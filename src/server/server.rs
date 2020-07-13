use std::collections::HashMap;
use std::time::SystemTime;

use crate::grid::Coords;
use crate::grid::Dir;
use crate::grid::Grid;
use crate::robot::Robot;
use super::ServerConfig;

/// ARES Server internal state
pub struct Server {
    config: ServerConfig,
    grid: Grid,
    robots: HashMap<i64, Robot>,

    /// if true, we've been asked to shutdown
    shutdown: bool,
}

/// The ARES Server
impl Server {
    /// Create a new server; load all data from the DB
    pub fn new(config: ServerConfig) -> Server {
        let mut grid: Grid = Grid::load(&config.conn).expect("Failed to load grid");
        println!("Loaded grid with {} cells", grid.cells.len());

        let robots: HashMap<i64, Robot> = Robot::load_all(&config.conn);
        println!("Loaded {} active robots", robots.len());

        let mut robot_locs: HashMap<Coords, i64> = HashMap::new();
        for (id, robot) in &robots {
            robot_locs.insert(Coords{ q: robot.data.q, r: robot.data.r }, *id);
        }

        grid.robot_locs = robot_locs;

        Server { config, grid, robots, shutdown: false }
    }

    /// Spawn a new robot by finding an open, unoccupied cell
    fn spawn_robot(&mut self) {
        let coords = self.grid.get_random_open_cell();
        let orientation: Dir = rand::random();
        let robot = Robot::new(coords.clone(), orientation, Some(&self.config.conn));
        self.grid.robot_locs.insert(coords.clone(), robot.data.id);
        self.robots.insert(robot.data.id, robot);
    }

    /// The main run loop for the ARES server.  Spawns robots if needed; tick all the robot
    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        while !self.shutdown {
            if self.robots.len() < self.config.max_bots {
                self.spawn_robot();
            }

            for (id, robot) in &mut self.robots {
                println!("Tick robot: {}", id);
                robot.tick(&self.config.conn);
            }

            // Wait for remained of the tick time
            if let Ok(elapse) = last_tick.elapsed() {
                let sleep_time = std::time::Duration::from_secs(1) - elapse;
                std::thread::sleep(sleep_time);
                last_tick = SystemTime::now();
            } else {
                std::thread::sleep(std::time::Duration::from_secs(1));
                last_tick = SystemTime::now();
            }
            println!("Tick!");
        }
    }
}