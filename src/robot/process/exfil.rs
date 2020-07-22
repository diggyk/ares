use diesel::PgConnection;

use super::ProcessResult;
use super::*;

pub struct Exfil {}

impl Process for Exfil {
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        robot.tick_exfil_countdown(Some(conn));
        println!(
            "Robot {}: Countdown: {}",
            robot.data.id, robot.data.exfil_countdown
        );

        if robot.data.exfil_countdown == 0 {
            robot.destroy(Some(conn));
            ProcessResult::ServerRequest(Request::Exfiltrate {
                robot_id: robot.data.id,
            })
        } else {
            ProcessResult::Ok
        }
    }

    fn init(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Exfiltrate");
        robot.start_exfil_countdown(Some(conn));

        return ProcessResult::Ok;
    }
}
