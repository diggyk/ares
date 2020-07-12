use crate::db::DbClient;
use crate::grid::Grid;
use crate::robot::Robot;
use super::ServerConfig;

use std::collections::HashMap;

pub struct Server<'a> {
    config: ServerConfig,
    db: DbClient,
    grid: Grid,
    robots: HashMap<i64, Robot<'a>>,
}

impl<'a> Server<'a> {
    pub fn new(config: ServerConfig) -> Server<'a> {
        let db: DbClient = DbClient::new(
            &config.dbuser, 
            &config.dbpw, 
            &config.dbhost, 
            &config.dbname,
        );
        let grid: Grid = db.get_all_cells().into();
        let robots = HashMap::new();

        Server { config, db, grid, robots }
    }

    pub fn run(&self) {
        let shutdown = false;

        ctrlc::set_handler(move || {
            println!("Signal for shutdown");
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        while !shutdown {
            std::thread::sleep(std::time::Duration::from_secs(10));
            println!("Still alive");
        }
    }
}