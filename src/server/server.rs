use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::grid::Coords;
use crate::grid::Dir;
use crate::grid::Grid;
use crate::robot::Robot;
use super::ServerConfig;

/// ARES Server internal state
pub struct Server {
    config: ServerConfig,
    grid: Arc<Mutex<Grid>>,
    robots: HashMap<i64, Robot>,

    /// if true, we've been asked to shutdown
    shutdown: bool,
}

/// The ARES Server
impl Server {
    /// Create a new server; load all data from the DB
    pub fn new(config: ServerConfig) -> Server {
        let grid = Arc::new(Mutex::new(Grid::load(&config.conn).expect("Failed to load grid")));
        println!("Loaded grid with {} cells", grid.lock().unwrap().cells.len());

        let robots: HashMap<i64, Robot> = Robot::load_all(&config.conn, grid.clone());
        println!("Loaded {} active robots", robots.len());

        let mut robot_locs: HashMap<Coords, i64> = HashMap::new();
        for (id, robot) in &robots {
            robot_locs.insert(Coords{ q: robot.data.q, r: robot.data.r }, *id);
        }

        grid.lock().unwrap().robot_locs = robot_locs;

        Server { config, grid, robots, shutdown: false }
    }

    /// Spawn a new robot by finding an open, unoccupied cell
    fn spawn_robot(&mut self) {
        let mut grid = self.grid.lock().expect("Could not get lock on grid");
        let coords = grid.get_random_open_cell();
        let orientation: Dir = rand::random();
        let robot = Robot::new(coords.clone(), orientation, Some(&self.config.conn), self.grid.clone());
        grid.robot_locs.insert(coords.clone(), robot.data.id);
        self.robots.insert(robot.data.id, robot);
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

            for (_, robot) in &mut self.robots {
                robot.tick(&self.config.conn);
                robot.ident();
            }

            // Wait for remained of the tick time
            if let Ok(elapse) = last_tick.elapsed() {
                if elapse < Duration::from_secs(1) {
                    let sleep_time = std::time::Duration::from_secs(1) - elapse;
                    std::thread::sleep(sleep_time);
                    last_tick = SystemTime::now();
                } else {
                    println!("{:?}", elapse);
                    last_tick = SystemTime::now();
                }
                std::thread::sleep(Duration::from_secs(1));
            } else {
                std::thread::sleep(Duration::from_secs(1));
                last_tick = SystemTime::now();
            }

            // self.wait_for_enter().expect("Not possible");
        }
    }
}