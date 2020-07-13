use std::collections::HashMap;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::utils;
use crate::grid::*;
use crate::schema::robots;
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

pub struct Robot {
    pub data: RobotData,
    pub active_process: Option<Processes>,
}

impl Robot {
    /// Load all the robots out of the database
    pub fn load_all(conn: &PgConnection) -> HashMap<i64, Robot> {
        let mut _robots = HashMap::new();
        let results = robots::table.load::<RobotData>(conn).expect("Failed to load robots");
        
        for result in results {
            _robots.insert(result.id, Robot {
                data: result,
                active_process: None,
            });
        }

        _robots
    }

    /// Create a new robot at the specified coordinates with the specified orientation
    pub fn new(coords: Coords, orientation: Dir, conn: Option<&PgConnection>) -> Robot {
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
            data: _robot,
            active_process: None,
        }
    }

    /// Handles a tick 
    pub fn tick(&mut self, conn: &PgConnection) {
        if let None = self.active_process {
            self.active_process = Some(Processes::Neutral);
        }

        let process = self.active_process.as_ref().unwrap();

        let result = process.run(conn);
        println!("{} {:?} = {:?}", self.data.name, process, result);
    }
}

#[cfg(test)]
#[test]
fn basic_robot_new() {
    let coords = Coords{ q: -2, r: 5};
    let dir = Dir::Orient120;

    let robot = Robot::new(coords, dir, None);

    assert_eq!(robot.data.q, -2);
    assert_eq!(robot.data.r, 5);
    assert_eq!(robot.data.orientation, Dir::Orient120);
}