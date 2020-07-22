use diesel::PgConnection;

use super::ProcessResult;
use super::*;
use crate::robot::modules::collector::*;

pub struct Collect {}

impl Process for Collect {
    /// Main run of the Collect process
    fn run(_: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        let collection_rate =
            CollectorModule::get_collection_rate(robot.modules.m_collector.as_str());

        let max_val_inventory =
            CollectorModule::get_collection_max(robot.modules.m_collector.as_str());

        // if we have mined all we are allowed to mine in a single collection run
        // or we have mined all our collector can carry then
        // we should transition back to neutral
        if robot.data.mined_amount >= collection_rate * 10
            || robot.data.val_inventory == max_val_inventory
        {
            println!(
                "Robot {} has collected max amount for this iteration",
                robot.data.id
            );
            return ProcessResult::TransitionToNeutral;
        }

        let amount_to_mine;
        if max_val_inventory - robot.data.val_inventory < collection_rate {
            amount_to_mine = max_val_inventory - robot.data.val_inventory;
        } else {
            amount_to_mine = collection_rate;
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
            amount: amount_to_mine,
        });
    }

    // initialize this process
    fn init(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Collect");
        robot.start_new_mining_operation(Some(conn));

        return ProcessResult::Ok;
    }
}
