use std::collections::HashMap;
use std::time::SystemTime;

use crate::grid::Dir;
use crate::grid::Grid;
use crate::robot::Robot;
use super::ServerConfig;

pub struct Server {
    config: ServerConfig,
    grid: Grid,
    robots: HashMap<i64, Robot>,

    shutdown: bool,
}

impl Server {
    pub fn new(config: ServerConfig) -> Server {
        let grid: Grid = Grid::load(&config.conn).expect("Failed to load grid");

        println!("Loaded grid with {} cells", grid.cells.len());

        let robots: HashMap<i64, Robot> = Robot::load_all(&config.conn);
        
        println!("Loaded {} active robots", robots.len());

        Server { config, grid, robots, shutdown: false }
    }

    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        while !self.shutdown {
            if self.robots.len() < self.config.max_bots {
                let coords = self.grid.get_random_open_cell();
                let orientation: Dir = rand::random();
                println!("{:?} {:?}", coords, orientation);
                let mut robot = Robot::new(coords, orientation, Some(&self.config.conn));
                self.robots.insert(robot.id, robot);
            }
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