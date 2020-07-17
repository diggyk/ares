use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::grid::*;
use crate::robot::*;
use crate::grid::utils::traversal::is_reachable;
use super::*;

/// Holds information about robots visible from the last scan
#[derive(Clone, Debug, PartialEq)]
pub struct VisibleRobot {
    pub robot_id: i64,
    pub coords: Coords,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisibleValuable {
    pub valuable_id: i64,
    pub coords: Coords,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScanResults {
    pub scanned_cells: Vec<Coords>,
    pub visible_robots: Vec<VisibleRobot>,
    pub visible_valuables: Vec<VisibleValuable>,
}

pub struct Scan {}

impl Process for Scan {
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        let our_coords = Coords{ q: robot.data.q, r: robot.data.r };
        let grid = robot.grid.lock().unwrap();

        // For now, let's scan in a 120 for distance of 2
        let cells = grid.get_cells(&our_coords, robot.data.orientation, 120, 1);
        let cells_full: HashMap<Coords, GridCell> = cells.iter().map(
            |cell| (Coords{ q: cell.q, r: cell.r }, *cell.to_owned())
        ).collect();

        let mut known_cells: Vec<RobotKnownCell> = Vec::new();
        let mut scanned_cells: Vec<Coords> = Vec::new();
        let mut visible_robots: Vec<VisibleRobot> = Vec::new();
        let mut visible_valuables: Vec<VisibleValuable> = Vec::new();
        for cell in cells {
            // TODO: see if this cell is visible from this starting location
            let cell_coords = Coords{q: cell.q, r: cell.r};
            let distance = our_coords.distance_to(&cell_coords);
            let visible = is_reachable(&our_coords, &cell_coords, &cells_full, distance);
            
            // if visible (or is the location we are standing on), add it to known cells
            if distance == 0 || visible {
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

                // also see if there is a robot in that cell
                let other_robot = grid.get_robot_by_loc(&Coords{q: cell.q, r: cell.r});
                if other_robot.is_some() {
                    visible_robots.push(VisibleRobot {
                        robot_id: *other_robot.unwrap(),
                        coords: Coords{q: cell.q, r: cell.r},
                    });
                }
            }
        }

        drop(grid);
        robot.update_known_cells(conn, known_cells);
        robot.update_visible_others(&visible_robots);

        ProcessResult::ScannedCells(ScanResults{
            scanned_cells,
            visible_robots,
            visible_valuables,
        })
    }

    // transition 
    fn init(_: &PgConnection, _: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        ProcessResult::Ok
    }
}