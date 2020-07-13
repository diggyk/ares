use std::collections::HashMap;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::utils;
use crate::grid::*;
use crate::schema::robots;

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
pub struct Robot {
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

impl Robot {
    pub fn load_all(conn: &PgConnection) -> HashMap<i64, Robot> {
        let mut _robots = HashMap::new();
        let results = robots::table.load::<Robot>(conn).expect("Failed to load robots");
        
        for result in results {
            _robots.insert(result.id, result);
        }

        _robots
    }
    pub fn new(coords: Coords, orientation: Dir, conn: Option<&PgConnection>) -> Robot {
        let new_robot = NewRobot {
            name: utils::random_string(8),
            q: coords.q,
            r: coords.r,
            orientation,
        };

        let mut _robot: Robot;
        if let Some(conn) = conn {
            _robot = diesel::insert_into(robots::table)
                .values(new_robot)
                .get_result(conn).expect("Error saving cells");
        } else {
            _robot = Robot {
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

        println!("{:#?}", _robot);
        _robot
    }
}

#[cfg(test)]
#[test]
fn basic_robot_new() {
    let coords = Coords{ q: -2, r: 5};
    let dir = Dir::Orient120;

    let robot = Robot::new(coords, dir, None);

    assert_eq!(robot.q, -2);
    assert_eq!(robot.r, 5);
    assert_eq!(robot.orientation, Dir::Orient120);
}