use diesel::PgConnection;

use super::ProcessResult;
use super::*;
use crate::grid::utils::traversal;
use crate::robot::modules::weapon::WeaponModule;
use crate::robot::*;

pub struct Pursue {}

impl Process for Pursue {
    fn init(
        conn: &PgConnection,
        robot: &mut Robot,
        message: Option<ProcessResult>,
    ) -> ProcessResult {
        robot.movement_queue = None;

        let target_id: i64;
        let mut target_coords: Option<Coords> = None;

        // We have to get a message containing the process result of a process
        // that decided we must move
        if let None = message {
            return ProcessResult::Fail;
        }

        let message = message.unwrap();
        match message {
            ProcessResult::TransitionToPursue(id) => {
                target_id = id;
                println!("Robot {}: Pursue Robot {}", robot.data.id, id);
                robot.set_status_text(Some(conn), &format!("I'm pursuing Robot {}.", id));
            }
            _ => return ProcessResult::Fail,
        }

        // find the coords of the other robot
        for other in &robot.visible_others {
            if other.robot_id == target_id {
                target_coords = Some(other.coords);
            }
        }

        if target_coords.is_none() {
            return ProcessResult::Fail;
        }

        // find the path to the other robot
        let moves = traversal::find_path(robot, target_coords.unwrap(), true);
        match moves {
            Ok(path_queue) => robot.movement_queue = Some(path_queue),
            Err(s) => {
                println!("{}", s);
                return ProcessResult::Fail;
            }
        }

        robot.update_pursuit_details(Some(conn), target_id, &target_coords.unwrap());

        ProcessResult::Ok
    }

    /// Main run of the Pursue process
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        let robot_coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };

        let current_target_coords = Coords {
            q: robot.data.pursuit_last_q,
            r: robot.data.pursuit_last_r,
        };

        // find the path to the other robot
        let moves = traversal::find_path(robot, current_target_coords.clone(), true);

        if moves.is_err() {
            return ProcessResult::TransitionToNeutral;
        }
        robot.movement_queue = Some(moves.unwrap());

        // take a step
        let power_need = drivesystem::DriveSystemModule::get_power_usage(&robot.modules.m_power);
        if robot.data.power < power_need {
            return ProcessResult::OutOfPower;
        }
        robot.use_power(Some(conn), power_need);

        // try to move but don't care if we fail (we might be right next to the target)
        robot.move_robot(conn);

        // scan and make sure we still see our robot
        let mut _visible_robots: Vec<VisibleRobot> = Vec::new();
        let scan_results = Scan::run(conn, robot, None);
        if let ProcessResult::ScannedCells(scan_results) = scan_results {
            _visible_robots = scan_results.visible_robots;
        } else if scan_results == ProcessResult::OutOfPower {
            return ProcessResult::OutOfPower;
        }

        let mut latest_coords: Option<Coords> = None;
        // if we can no longer see the robot we want to pursue,
        // switch to Move and to last known coords
        for _robot in _visible_robots {
            if _robot.robot_id == robot.data.pursuit_id {
                latest_coords = Some(_robot.coords);
            }
        }
        if latest_coords.is_none() {
            return ProcessResult::TransitionToMove(
                current_target_coords,
                Dir::get_random(),
                false,
            );
        }

        // we still see the target so update our info on the target
        robot.update_pursuit_details(Some(conn), robot.data.pursuit_id, &latest_coords.unwrap());

        let in_range = WeaponModule::in_range(
            &robot.modules.m_weapons,
            &robot_coords,
            &robot.data.orientation,
            &latest_coords.unwrap(),
        );
        if in_range {
            // TODO: make sure this is within the FOV
            robot.set_status_text(
                Some(conn),
                &format!("I would fire on {}", robot.data.pursuit_id),
            );
        }

        ProcessResult::Ok
    }
}
