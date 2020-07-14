use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::utils;
use crate::grid::*;
use crate::schema::*;
use super::process::*;

#[derive(Debug, Queryable, Insertable)]
#[table_name="robots"]
pub struct NewRobot {
    pub name: String,
    pub q: i32,
    pub r: i32,
    pub orientation: Dir,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name="robots"]
pub struct RobotData {
    pub id: i64,
    pub name: String,

    pub owner: Option<i32>,
    pub affiliation: Option<i32>,
    pub q: i32,
    pub r: i32,
    pub orientation: Dir,
    pub gridcell: Option<i32>,
    pub components: Option<serde_json::Value>,
    pub configs: Option<serde_json::Value>,
}

/// Represents a grid cell that is known by a robot
#[derive(Debug, Queryable, Insertable)]
#[table_name="robot_known_cells"]
pub struct RobotKnownCell {
    pub robot_id: i64,
    pub gridcell_id: i32,
    pub discovery_time: std::time::SystemTime,
}

impl RobotKnownCell {
    /// Load all the known grid cells for a robot out of memory
    pub fn load_all(conn: &PgConnection, robot_id: i64) -> Vec<RobotKnownCell> {
        let results = robot_known_cells::table
            .filter(robot_known_cells::robot_id
            .eq(robot_id)).load::<RobotKnownCell>(conn);

        if let Ok(cells) = results {
            cells
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Robot {
    pub grid: Arc<Mutex<Grid>>,
    pub data: RobotData,
    pub known_cells: Vec<RobotKnownCell>,
    pub active_process: Option<Processes>,
}

impl Robot {
    /// Load all the robots out of the database
    pub fn load_all(conn: &PgConnection, grid: Arc<Mutex<Grid>>) -> HashMap<i64, Robot> {
        let mut _robots = HashMap::new();
        let results = robots::table.load::<RobotData>(conn).expect("Failed to load robots");
        
        for result in results {
            let id = result.id;
            let mut robot = Robot {
                grid: grid.clone(),
                data: result,
                known_cells: RobotKnownCell::load_all(conn, id),
                active_process: None,
            };

            if let Ok(known_cells) = robot_known_cells::table.filter(
                robot_known_cells::robot_id.eq(id)
            ).load::<RobotKnownCell>(conn) {
                robot.known_cells = known_cells;
            }
            _robots.insert(id, robot);
        }

        _robots
    }

    /// Create a new robot at the specified coordinates with the specified orientation
    pub fn new(coords: Coords, orientation: Dir, conn: Option<&PgConnection>, grid: Arc<Mutex<Grid>>) -> Robot {
        let new_robot = NewRobot {
            name: utils::random_string(8),
            q: coords.q,
            r: coords.r,
            orientation,
        };

        let mut _robot: RobotData;
        if let Some(conn) = conn {
            _robot = diesel::insert_into(robots::table)
                .values(new_robot)
                .get_result(conn).expect("Error saving cells");
        } else {
            _robot = RobotData {
                id: 0,
                name: utils::random_string(8),
                owner: None,
                affiliation: None,
                q: coords.q,
                r: coords.r,
                orientation,
                gridcell: None,
                components: None,
                configs: None,
            }
        }

        Robot {
            grid,
            data: _robot,
            known_cells: Vec::new(),
            active_process: None,
        }
    }

    /// Handles a tick 
    pub fn tick(&mut self, conn: &PgConnection) {
        if let None = self.active_process {
            self.active_process = Some(Processes::Neutral);
        }

        let process = self.active_process.as_ref().unwrap().clone();

        let result = match process {
            Processes::Neutral => Neutral::run(conn, self),
            Processes::Scan => ProcessResult::Ok,
        };

        println!("{:?}", result);
    }
}