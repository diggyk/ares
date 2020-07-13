use diesel::PgConnection;

use super::Process;
use super::Processes;
use super::ProcessResult;

pub struct Neutral {}

impl Process for Neutral {
    fn run(conn: &PgConnection) -> ProcessResult {
        ProcessResult::Ok
    }
}