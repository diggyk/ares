use diesel::PgConnection;

use super::*;

pub struct Neutral {}

/// The "Neutral" process is when there is no active fleeing, mining, or exploring going on
impl Process for Neutral {
    fn run(conn: &PgConnection, robot: &mut Robot) -> ProcessResult {
        Scan::run(conn, robot)
    }
}