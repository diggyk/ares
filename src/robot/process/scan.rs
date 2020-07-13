use diesel::PgConnection;

use crate::grid::*;
use super::*;

pub struct Scan {}

impl Process for Scan {
    fn run(conn: &PgConnection, robot: &mut Robot) -> ProcessResult {
        let our_coords = Coords{ q: robot.data.q, r: robot.data.r };
        

        ProcessResult::Ok
    }
}