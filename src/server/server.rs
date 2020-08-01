use rand::Rng;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use super::*;
use crate::grid::Coords;
use crate::grid::Dir;
use crate::grid::Grid;
use crate::robot::modules::*;
use crate::robot::Robot;
use crate::utils;
use crate::valuable::*;

/// ARES Server internal state
pub struct Server {
    config: ServerConfig,
    grid: Arc<Mutex<Grid>>,
    robots: HashMap<i64, Robot>,
    valuables: HashMap<i64, Valuable>,

    /// if true, we've been asked to shutdown
    shutdown: bool,
}

/// The ARES Server
impl Server {
    /// Create a new server; load all data from the DB
    pub fn new(config: ServerConfig) -> Server {
        let grid = Arc::new(Mutex::new(
            Grid::load(&config.conn).expect("Failed to load grid"),
        ));
        println!(
            "Loaded grid with {} cells",
            grid.lock().unwrap().cells.len()
        );

        let robots: HashMap<i64, Robot> = Robot::load_all(&config.conn, grid.clone());
        println!("Loaded {} active robots", robots.len());

        for (_, robot) in &robots {
            grid.lock().unwrap().add_robot(robot);
        }

        let valuables: HashMap<i64, Valuable> = Valuable::load_all(&config.conn);
        let mut valuables_locs: HashMap<Coords, i64> = HashMap::new();
        for (id, valuable) in &valuables {
            valuables_locs.insert(
                Coords {
                    q: valuable.q,
                    r: valuable.r,
                },
                *id,
            );
        }

        grid.lock().unwrap().valuables_locs = valuables_locs;

        Server {
            config,
            grid,
            robots,
            valuables,
            shutdown: false,
        }
    }

    /// Spawn a new robot by finding an open, unoccupied cell
    fn spawn_robot(&mut self) {
        let mut grid = self.grid.lock().expect("Could not get lock on grid");
        let coords = grid.get_random_open_cell();
        let orientation: Dir = rand::random();

        let mut modules: HashMap<String, String> = HashMap::new();
        let scanner_module = scanner::ScannerModule::get_random();
        modules.insert("m_scanner".to_string(), scanner_module.to_string());

        let memory_module = match scanner_module.as_str() {
            "basic" => "basic",
            "plus" => "basic",
            "triscan" => "basic",
            "triscan_advanced" => "plus",
            "triscan_ultra" => "plus",
            "boxium_started" => "plus",
            "boxium_advanced" => "ikito",
            "boxium_ultra" => "jindai",
            "omni_basic" => "jindai",
            "omni_ultra" => "jindai",
            _ => "basic",
        };

        modules.insert("m_memory".to_string(), memory_module.to_string());

        let collector_module = collector::CollectorModule::get_random();
        modules.insert("m_collector".to_string(), collector_module.to_string());

        let mut power_module = power::PowerModule::get_random();
        if collector_module == "ultratech" && power_module == "basic" {
            power_module = "plus".to_string();
        }
        modules.insert("m_power".to_string(), power_module.to_string());

        let weapon_module = weapon::WeaponModule::get_random();
        modules.insert("m_weapon".to_string(), weapon_module.to_string());

        let robot = Robot::new(
            coords.clone(),
            orientation,
            Some(&self.config.conn),
            self.grid.clone(),
            Some(modules),
        );

        grid.add_robot(&robot);

        self.robots.insert(robot.data.id, robot);
    }

    /// Spawn a new valuable at a given location for a given amount
    fn spawn_valuable(&mut self, coords: &Coords, amount: i32) {
        let mut grid = self.grid.lock().expect("Could not get lock on grid");
        if let Some(valuable_id) = grid.valuables_locs.get(coords) {
            let valuable = self.valuables.get_mut(valuable_id);
            if valuable.is_some() {
                valuable.unwrap().add_to_amount(&self.config.conn, amount);
            }
        } else {
            let valuable = Valuable::new(coords.clone(), amount, Some(&self.config.conn));
            grid.valuables_locs.insert(coords.clone(), valuable.id);
            self.valuables.insert(valuable.id, valuable);
        }
    }

    /// Spawn a new valuable in a random open location with a random amount
    fn spawn_random_valuable(&mut self) {
        let mut grid = self.grid.lock().expect("Could not get lock on grid");
        let coords = grid.get_random_open_cell();
        let mut rng = rand::thread_rng();
        let amount: i32 = rng.gen_range(50, 5000);
        drop(grid);

        self.spawn_valuable(&coords, amount);
    }

    /// Check all the valuables; if they are now exhausted, tell them
    /// to destroy themselves and remove references to them in Server and Grid
    fn destroy_depleted_valuables(&mut self) {
        // see if the valuables have been exhausted and delete them
        let mut deleted_valuables: Vec<(Coords, i64)> = Vec::new();
        for (id, valuable) in &mut self.valuables {
            if valuable.amount == 0 {
                if valuable.destroy(&self.config.conn) == true {
                    deleted_valuables.push((
                        Coords {
                            q: valuable.q,
                            r: valuable.r,
                        },
                        *id,
                    ));
                }
            }
        }

        // delete any destroyed/depleted valubles from
        // our local tracking and the grid
        for valuable in deleted_valuables {
            self.grid
                .lock()
                .unwrap()
                .remove_valuable_by_loc(&valuable.0);
            self.valuables.remove(&valuable.1);
        }
    }

    fn _wait_for_enter(&self) -> std::io::Result<()> {
        println!("Paused (press enter)...");
        let mut buffer = [0; 1];
        std::io::stdin().read(&mut buffer)?;
        Ok(())
    }

    /// Mine a valuable specified by id for a given amount
    /// Return the amount mined in a server response
    fn mine_for_robot(
        &mut self,
        _robot_id: &i64,
        valuable_id: i64,
        amount: i32,
    ) -> Option<Response> {
        let valuable = self.valuables.get_mut(&valuable_id);

        if valuable.is_none() {
            println!("Valuable is non-existant");
            return Some(Response::Fail);
        }

        let mined_amount = valuable.unwrap().mine(&self.config.conn, amount);

        Some(Response::Mined {
            valuable_id,
            amount: mined_amount,
        })
    }

    /// Exfiltrate the robot by removing it from the grid and our own list of robots
    fn exfiltrate_robot(&mut self, robot_id: &i64) -> Option<Response> {
        let mut grid = self.grid.lock().unwrap();
        grid.remove_robot_by_id(robot_id);

        self.robots.remove(robot_id);

        None
    }

    /// Explode the robot by removing it from the grid and our own list of robots
    /// and creating a valuable at its location
    fn explode(&mut self, robot_id: &i64, valuables: i32) -> Option<Response> {
        let mut grid = self.grid.lock().unwrap();
        grid.remove_robot_by_id(robot_id);
        drop(grid);

        let robot = self.robots.remove(robot_id);

        let coords: Coords;
        if robot.is_some() {
            coords = Coords {
                q: robot.as_ref().unwrap().data.q,
                r: robot.as_ref().unwrap().data.r,
            };
        } else {
            return None;
        }

        self.spawn_valuable(&coords, valuables);

        None
    }

    fn attack_robot(&mut self, attacker_id: &i64, target_id: &i64) -> Option<Response> {
        let mut rng = rand::thread_rng();

        let attacker = &self.robots.get(attacker_id);
        if attacker.is_none() {
            return None;
        }

        let attacker_coords = attacker.as_ref().unwrap().get_coords();

        let min_power =
            weapon::WeaponModule::get_min_damage(&attacker.as_ref().unwrap().modules.m_weapons);
        let max_power =
            weapon::WeaponModule::get_max_damage(&attacker.as_ref().unwrap().modules.m_weapons);

        let damage = rng.gen_range(min_power, max_power + 1);

        // inflict damage to the target and then find the direction of the attack
        // and register that as well
        let mut target = self.robots.get_mut(target_id);
        let target_coords = target.as_ref().unwrap().get_coords();
        let mut attack_dir = utils::get_bearing(&Dir::Orient0, &target_coords, &attacker_coords);
        if attack_dir.is_none() {
            attack_dir = Some(Dir::get_random().into());
        }

        println!(
            "Server: received attack from {} to {} for {}; bearing from target to attacker: {}",
            attacker_id,
            target_id,
            damage,
            attack_dir.unwrap()
        );

        target
            .as_mut()
            .unwrap()
            .update_hull_strength(Some(&self.config.conn), -1 * damage);
        target.as_mut().unwrap().record_attack(
            Some(&self.config.conn),
            *attacker_id,
            attack_dir.unwrap(),
        );

        Some(Response::AttackSuccess {
            target_id: *target_id,
            damage,
        })
    }

    /// When we tick a robot, it may ask the server to do something
    /// This is usually because robots do not have direct access to other
    /// robots or valuables.  So it must ask the server to do things like
    /// mining or shooting at others
    fn handle_request_for_robot(&mut self, robot_id: &i64, request: Request) -> Option<Response> {
        match request {
            Request::Attack { target_id } => self.attack_robot(&robot_id, &target_id),
            Request::Exfiltrate { robot_id } => self.exfiltrate_robot(&robot_id),
            Request::Explode { valuables } => self.explode(&robot_id, valuables),
            Request::Mine {
                valuable_id,
                amount,
            } => self.mine_for_robot(robot_id, valuable_id, amount),
        }
    }

    fn tick_robot(&mut self, robot_id: &i64) {
        // the robot may send back a request for this server to perform
        let robot = self.robots.get_mut(robot_id);

        if robot.is_none() {
            return ();
        }

        let _robot = robot.unwrap();

        let server_request = _robot.tick(&self.config.conn);

        let server_request = if server_request.is_some() {
            server_request.unwrap()
        } else {
            return ();
        };

        let server_response = self.handle_request_for_robot(robot_id, server_request);
        if server_response.is_some() {
            let robot = self.robots.get_mut(robot_id);
            robot
                .unwrap()
                .handle_server_response(Some(&self.config.conn), server_response.unwrap());
        }
    }

    /// The main run loop for the ARES server.  Spawns robots if needed; tick all the robot
    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        while !self.shutdown {
            if self.config.debug {
                self._wait_for_enter().expect("Not possible");
            }

            while self.robots.len() < self.config.max_bots {
                self.spawn_robot();
            }

            while self.valuables.len() < self.config.max_valuables {
                self.spawn_random_valuable();
            }

            // because we need the server `self` to be mutable, we cannot borrow
            // anything else to send along, otherwise, we get hit by the borrower
            // check.  So, let's make copies of the robot ids and use that
            let robot_ids: Vec<i64> = self.robots.keys().map(|k| k.clone()).collect();
            for id in robot_ids {
                self.tick_robot(&id);
            }

            self.destroy_depleted_valuables();

            // Wait for remainer of the tick time
            if let Ok(elapse) = last_tick.elapsed() {
                if elapse < Duration::from_secs(1) {
                    let sleep_time = std::time::Duration::from_secs(1) - elapse;
                    std::thread::sleep(sleep_time);
                    last_tick = SystemTime::now();
                } else {
                    last_tick = SystemTime::now();
                }
                // TODO fix this by addressing the possible overflow
                std::thread::sleep(Duration::from_millis(500));
            } else {
                std::thread::sleep(Duration::from_millis(500));
                last_tick = SystemTime::now();
            }
        }
    }
}
