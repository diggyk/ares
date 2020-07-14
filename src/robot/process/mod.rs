use diesel::PgConnection;

mod neutral;
mod scan;

pub use neutral::Neutral;
pub use scan::Scan;

use crate::grid::*;
use super::Robot;


/// Message returned from a process run to let robot know what to do next
#[derive(Debug)]
pub enum ProcessResult {
    /// Ok means the process succeeded and should remain the active process
    Ok,
    /// Indicates the process failed
    Fail,
    /// Result of a scan
    ScannedCells(Vec<Coords>),
    /// Indicate a switch to Move; the last bool means to spin 180 at the end
    TransitionToMove(Option<Coords>, Option<Dir>, bool),
}

/// List of all the processes with helpers to run the process
#[derive(Debug)]
pub enum Processes {
    Neutral,
    Scan,
}

/// Trait to define a process
pub trait Process {
    fn run(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult;
}