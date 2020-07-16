use diesel::PgConnection;

use crate::grid::utils::traversal;
use super::*;
use super::ProcessResult;

pub struct Move {}

/// The "Move" process involves a movement queue of moves
impl Process for Move {
    /// Main run of the Neutral process
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        // Take the next X moves based on the drive system
        robot.move_robot(conn);

        let mut scanned_cells: Vec<Coords> = Vec::new();
        if let ProcessResult::ScannedCells(cells) = Scan::run(conn, robot, None) {
            scanned_cells = cells;
        }
        // println!("Scanned {} cells", scanned_cells.len());

        if robot.movement_queue.is_none() {
            robot.movement_queue = None;
            return ProcessResult::TransitionToNeutral;
        } else {
            return ProcessResult::Ok;
        }
    }

    fn init(_: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult {

        robot.movement_queue = None;

        let robot_coords = Coords{ q: robot.data.q, r: robot.data.r };

        let target_coords: Coords;
        let orientation: Dir;
        let spin: bool;

        // We have to get a message containing the process result of a process
        // that decided we must move
        if let None = message {
            return ProcessResult::Fail;
        }

        let message = message.unwrap();
        match message {
            ProcessResult::TransitionToMove(tc, o, s) => {
                target_coords = tc;
                orientation = o;
                spin = s;
                println!("Move to {:?}, {:?}, {:?}", &target_coords, &orientation, spin);
            },
            _ => {
                return ProcessResult::Fail
            },
        }

        if target_coords == robot_coords {
            robot.movement_queue = Some(traversal::find_spin(robot.data.orientation, orientation));
        } else {
            let moves = traversal::find_path(robot, target_coords);
            match moves {
                Ok(path_queue) => robot.movement_queue = Some(path_queue),
                Err(s) => {
                    println!("{}", s);
                    return ProcessResult::Fail;
                }
            }
        }

        ProcessResult::Ok
    }
}