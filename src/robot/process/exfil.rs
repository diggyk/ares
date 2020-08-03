use diesel::PgConnection;

use super::ProcessResult;
use super::*;

pub struct Exfil {}

impl Process for Exfil {
    fn run(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        _: Option<ProcessResult>,
    ) -> ProcessResult {
        robot.tick_exfil_countdown(conn);

        robot.set_status_text(
            conn,
            format!(
                "I'm calling for exfiltration in ... {}",
                robot.data.exfil_countdown,
            )
            .as_str(),
        );

        if robot.data.exfil_countdown == 0 {
            robot.destroy(conn);
            ProcessResult::ServerRequest(Request::Exfiltrate {
                robot_id: robot.data.id,
            })
        } else {
            ProcessResult::Ok
        }
    }

    fn init(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        _: Option<ProcessResult>,
    ) -> ProcessResult {
        println!("Transition to Exfiltrate");
        robot.set_status_text(conn, "I'm calling for exfiltration.");
        robot.start_exfil_countdown(conn);

        return ProcessResult::Ok;
    }
}
