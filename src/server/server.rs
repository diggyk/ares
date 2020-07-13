use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::db::DbClient;
use crate::grid::Dir;
use crate::grid::Grid;
use crate::robot::Robot;
use super::ServerConfig;

pub struct Server {
    config: ServerConfig,
    db: Arc<Mutex<DbClient>>,
    grid: Grid,
    robots: HashMap<i64, Robot>,

    shutdown: bool,
}

impl Server {
    pub fn new(config: ServerConfig) -> Server {
        let mut db: DbClient = DbClient::new(
            &config.dbuser, 
            &config.dbpw, 
            &config.dbhost, 
            &config.dbname,
        );
        let db = Arc::new(Mutex::new(db));
        let grid: Grid = db.lock().unwrap().get_all_cells().into();
        let robots = HashMap::new();

        Server { config, db, grid, robots, shutdown: false }
    }

    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        while !self.shutdown {
            if self.robots.len() < self.config.max_bots {
                let coords = self.grid.get_random_open_cell();
                let orientation: Dir = rand::random();
                println!("{:?} {:?}", coords, orientation);
                let mut robot = Robot::new(coords, orientation);
                robot.attach_db(Arc::clone(&self.db));
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