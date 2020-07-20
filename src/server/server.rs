use rand::Rng;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use super::ServerConfig;
use crate::grid::Coords;
use crate::grid::Dir;
use crate::grid::Grid;
use crate::robot::modules::*;
use crate::robot::Robot;
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

        let mut robot_locs: HashMap<Coords, i64> = HashMap::new();
        for (id, robot) in &robots {
            robot_locs.insert(
                Coords {
                    q: robot.data.q,
                    r: robot.data.r,
                },
                *id,
            );
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

        grid.lock().unwrap().robot_locs = robot_locs;

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

        let robot = Robot::new(
            coords.clone(),
            orientation,
            Some(&self.config.conn),
            self.grid.clone(),
            Some(modules),
        );
        grid.robot_locs.insert(coords.clone(), robot.data.id);
        self.robots.insert(robot.data.id, robot);
    }

    /// Spawn a new valuable in a random open location with a random amount
    fn spawn_valuable(&mut self) {
        let mut grid = self.grid.lock().expect("Could not get lock on grid");
        let coords = grid.get_random_open_cell();
        let mut rng = rand::thread_rng();
        let amount: i32 = rng.gen_range(100, 10000);

        let valuable = Valuable::new(coords.clone(), amount, Some(&self.config.conn));

        grid.valuables_locs.insert(coords.clone(), valuable.id);
        self.valuables.insert(valuable.id, valuable);
    }

    fn _wait_for_enter(&self) -> std::io::Result<()> {
        println!("Paused (press enter)...");
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Ok(())
    }

    /// The main run loop for the ARES server.  Spawns robots if needed; tick all the robot
    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        while !self.shutdown {
            if self.robots.len() < self.config.max_bots {
                self.spawn_robot();
            }

            if self.valuables.len() < self.config.max_valuables {
                self.spawn_valuable();
            }

            for (_, robot) in &mut self.robots {
                robot.ident();
                robot.tick(&self.config.conn);
            }

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
                std::thread::sleep(Duration::from_secs(1));
            } else {
                std::thread::sleep(Duration::from_secs(1));
                last_tick = SystemTime::now();
            }

            // self._wait_for_enter().expect("Not possible");
        }
    }
}
