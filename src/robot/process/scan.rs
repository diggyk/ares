use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::grid::*;
use crate::robot::*;
use crate::schema::*;
use super::*;

pub struct Scan {}

impl Process for Scan {
    fn run(conn: &PgConnection, robot: &mut Robot) -> ProcessResult {
        let our_coords = Coords{ q: robot.data.q, r: robot.data.r };
        let grid = robot.grid.lock().unwrap();

        // For now, let's scan in a 120 for distance of 2
        let cells = grid.get_cells(our_coords, robot.data.orientation, 120, 1);

        let mut known_cells: Vec<RobotKnownCell> = Vec::new();
        for cell in cells {
            known_cells.push(
                RobotKnownCell {
                    robot_id: robot.data.id,
                    gridcell_id: cell.id,
                    discovery_time: SystemTime::now(),
                }
            )
        }

        println!("{:#?}", known_cells);

        let query = diesel::insert_into(robot_known_cells::table).values(known_cells)
            .on_conflict((robot_known_cells::robot_id, robot_known_cells::gridcell_id))
            .do_update().set(robot_known_cells::discovery_time.eq(SystemTime::now()))
            .execute(conn).expect("Could not store known cells");

        ProcessResult::Ok
    }
}