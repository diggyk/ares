use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::grid::*;
use crate::robot::*;
use crate::schema::*;
use crate::grid::utils::traversal::is_reachable;
use super::*;

pub struct Scan {}

impl Process for Scan {
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        let our_coords = Coords{ q: robot.data.q, r: robot.data.r };
        let grid = robot.grid.lock().unwrap();

        // For now, let's scan in a 120 for distance of 2
        let cells = grid.get_cells(&our_coords, robot.data.orientation, 120, 1);
        let cells_full: HashMap<Coords, &GridCell> = cells.iter().map(
            |cell| (Coords{ q: cell.q, r: cell.r }, *cell)
        ).collect();

        let mut known_cells: Vec<RobotKnownCell> = Vec::new();
        let mut scanned_cells: Vec<Coords> = Vec::new();
        for cell in cells {
            // TODO: see if this cell is visible from this starting location
            let cell_coords = Coords{q: cell.q, r: cell.r};
            let distance = our_coords.distance_to(&cell_coords);
            let reachable = is_reachable(&our_coords, &cell_coords, &cells_full, distance);
            if distance == 0 || reachable {
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
        }

        drop(grid);
        robot.update_known_cells(conn, known_cells);

        ProcessResult::ScannedCells(scanned_cells)
    }

    // transition 
    fn init(_: &PgConnection, _: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        ProcessResult::Ok
    }
}