use diesel::PgConnection;

use super::ProcessResult;
use super::*;
use crate::grid::utils::traversal;
use crate::robot::*;

pub struct Move {}

/// The "Move" process involves a movement queue of moves
impl Process for Move {
    /// Main run of the Neutral process
    fn run(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        _: Option<ProcessResult>,
    ) -> ProcessResult {
        // make sure we have enough power to run the scanner
        let power_need = drivesystem::DriveSystemModule::get_power_usage(&robot.modules.m_power);
        if robot.data.power < power_need {
            return ProcessResult::OutOfPower;
        }
        robot.use_power(conn, power_need);

        // Take the next move based on the drive system
        if let ProcessResult::Fail = robot.move_robot(conn) {
            return ProcessResult::TransitionToNeutral;
        }

        // we scan only so we can react to other robots
        let mut _visible_robots: Vec<VisibleRobot> = Vec::new();
        let scan_results = Scan::run(conn, robot, None);
        if let ProcessResult::ScannedCells(scan_result) = scan_results {
            _visible_robots = scan_result.visible_robots;
        } else if scan_results == ProcessResult::OutOfPower {
            return ProcessResult::OutOfPower;
        }

        if let Some(response) = robot.respond_to_others(conn) {
            return response;
        }

        // if we are not out of moves, switch to neutral
        if robot.movement_queue.is_none() {
            robot.movement_queue = None;
            return ProcessResult::TransitionToNeutral;
        } else {
            return ProcessResult::Ok;
        }
    }

    fn init(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        message: Option<ProcessResult>,
    ) -> ProcessResult {
        robot.movement_queue = None;

        let robot_coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };

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
                println!(
                    "Robot {}: Move to {:?}, {:?}, {:?}",
                    robot.data.id, &target_coords, &orientation, spin
                );
                robot.set_status_text(conn, format!("I'm moving to {},{}.", tc.q, tc.r).as_str());
            }
            ProcessResult::TransitionToFlee(tc, o) => {
                target_coords = tc;
                orientation = o;
                spin = true;
                println!(
                    "Robot {}: Flee to {:?}, {:?}, {:?}",
                    robot.data.id, &target_coords, &orientation, spin
                );
                robot.set_status_text(conn, format!("I'm fleeing to {},{}!", tc.q, tc.r).as_str());
            }
            _ => return ProcessResult::Fail,
        }

        if target_coords == robot_coords {
            robot.movement_queue = Some(traversal::find_spin(robot.data.orientation, orientation));
        } else {
            let moves = traversal::find_path(robot, target_coords, false);
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
