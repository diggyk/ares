use diesel::prelude::*;
use diesel::pg::PgConnection;
// use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::grid::*;
use crate::robot::*;
use crate::schema::*;
use super::*;

pub struct Scan {}

impl Process for Scan {
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        let our_coords = Coords{ q: robot.data.q, r: robot.data.r };
        let grid = robot.grid.lock().unwrap();

        // For now, let's scan in a 120 for distance of 2
        let cells = grid.get_cells(our_coords, robot.data.orientation, 120, 1);

        let mut known_cells: Vec<RobotKnownCell> = Vec::new();
        let mut scanned_cells: Vec<Coords> = Vec::new();
        for cell in cells {
            // TODO: see if this cell is visible from this starting location
            known_cells.push(
                RobotKnownCell {
                    robot_id: robot.data.id,
                    gridcell_id: cell.id,
                    discovery_time: SystemTime::now(),
                    q: cell.q,
                    r: cell.r,
                }
            );
            scanned_cells.push(Coords{q: cell.q, r: cell.r});
        }

        let query = diesel::insert_into(robot_known_cells::table).values(&known_cells)
            .on_conflict((robot_known_cells::robot_id, robot_known_cells::gridcell_id))
            .do_update().set(robot_known_cells::discovery_time.eq(SystemTime::now()))
            .execute(conn);

        if let Err(reason) = query {
            println!("Could not update known cells: {:?}", reason);
        }

        drop(grid);
        robot.update_known_cells(known_cells);

        ProcessResult::ScannedCells(scanned_cells)
    }

    // transition 
    fn init(_: &PgConnection, _: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        ProcessResult::Ok
    }
}