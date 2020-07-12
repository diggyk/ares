use std::collections::HashMap;
use std::time::SystemTime;

use crate::db::DbClient;
use crate::grid::Grid;
use crate::robot::Robot;
use super::ServerConfig;

pub struct Server<'a> {
    config: ServerConfig,
    db: DbClient,
    grid: Grid,
    robots: HashMap<i64, Robot<'a>>,

    shutdown: bool,
}

impl<'a> Server<'a> {
    pub fn new(config: ServerConfig) -> Server<'a> {
        let mut db: DbClient = DbClient::new(
            &config.dbuser, 
            &config.dbpw, 
            &config.dbhost, 
            &config.dbname,
        );
        let grid: Grid = db.get_all_cells().into();
        let robots = HashMap::new();

        Server { config, db, grid, robots, shutdown: false }
    }

    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        while !self.shutdown {
            if self.robots.len() < self.config.max_bots {

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