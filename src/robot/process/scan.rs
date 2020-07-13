use diesel::PgConnection;

use super::Process;
use super::Processes;
use super::ProcessResult;

pub struct Scan {}

impl Process for Scan {
    fn run(conn: &PgConnection) -> ProcessResult {
        ProcessResult::Ok
    }
}