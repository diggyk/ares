use diesel::PgConnection;

use super::ProcessResult;
use super::*;
use crate::robot::modules::*;

pub struct Explode {}

impl Process for Explode {
    fn run(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        _: Option<ProcessResult>,
    ) -> ProcessResult {
        robot.set_status_text(conn, "I'm dead!");

        let mut value = robot.data.val_inventory;
        value += power::PowerModule::get_max_power(&robot.modules.m_power);

        robot.destroy(conn);
        ProcessResult::ServerRequest(Request::Explode { valuables: value })
    }

    fn init(
        _: Option<&PgConnection>,
        robot: &mut Robot,
        _: Option<ProcessResult>,
    ) -> ProcessResult {
        println!("Robot {}: Transition to Explode", robot.data.id);
        return ProcessResult::Ok;
    }
}
