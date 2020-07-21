use diesel::PgConnection;

use super::ProcessResult;
use super::*;
use crate::robot::modules::collector::*;

pub struct Collect {}

impl Process for Collect {
    /// Main run of the Neutral process
    fn run(
        _: &PgConnection,
        robot: &mut Robot,
        previous_result: Option<ProcessResult>,
    ) -> ProcessResult {
        if previous_result.is_none() {
            return ProcessResult::TransitionToNeutral;
        }
        let previous_result = previous_result.unwrap();

        let mut mined: i32 = 0;
        let mut max_to_mine: i32 = 0;
        if let ProcessResult::Collected(prev_mined, prev_max_to_mine) = previous_result {
            mined = prev_mined;
            max_to_mine = prev_max_to_mine;
        }

        // find the valuable at this location

        // mine it
        // try to get the minable amount from the valuable
        mined += CollectorModule::get_collection_rate(robot.modules.m_collector.as_str());

        // if we've mined enough or if we are at capacity, transition to Neutral
        if mined > max_to_mine {
            return ProcessResult::Collected(mined, max_to_mine);
        }

        println!("Mined {} of {}", mined, max_to_mine);

        // else we return how much we've mined
        ProcessResult::Collected(mined, max_to_mine)
    }

    // initialize this process
    fn init(_: &PgConnection, _: &mut Robot, message: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Collect");
        if message.is_some() {
            if let Some(ProcessResult::TransitionToCollect(max_to_mine)) = message {
                return ProcessResult::Collected(0, max_to_mine);
            }
        }

        ProcessResult::Fail
    }
}
