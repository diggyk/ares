use diesel::PgConnection;

use super::ProcessResult;
use super::*;
use crate::robot::modules::collector::*;

pub struct Collect {}

impl Process for Collect {
    /// Main run of the Neutral process
    fn run(_: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        let collection_rate =
            CollectorModule::get_collection_rate(robot.modules.m_collector.as_str());

        if robot.data.mined_amount >= collection_rate * 10 {
            println!(
                "Robot {} has collected max amount for this iteration",
                robot.data.id
            );
            return ProcessResult::TransitionToNeutral;
        }

        let grid = robot.grid.lock().unwrap();
        let valuable = grid.get_valuable_id_by_loc(&Coords {
            q: robot.data.q,
            r: robot.data.r,
        });

        // if there is no valuable at this location (maybe depleted?)
        // switch back to neutral process
        if valuable.is_none() {
            return ProcessResult::TransitionToNeutral;
        }

        // otherwise, we need to ask the server to mine for us
        return ProcessResult::ServerRequest(Request::Mine {
            valuable_id: *valuable.unwrap(),
            amount: collection_rate,
        });
    }

    // initialize this process
    fn init(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Collect");
        robot.start_new_mining_operation(Some(conn));

        return ProcessResult::Ok;
    }
}
