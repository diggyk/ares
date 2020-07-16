use diesel::PgConnection;

mod moveproc;
mod neutral;
mod scan;

pub use moveproc::*;
pub use neutral::*;
pub use scan::*;

use crate::grid::*;
use super::Robot;


/// Message returned from a process run to let robot know what to do next
#[derive(Debug, PartialEq)]
pub enum ProcessResult {
    /// Ok means the process succeeded and should remain the active process
    Ok,
    /// Indicates the process failed
    Fail,
    /// Result of a scan
    ScannedCells(Vec<Coords>),
    /// Transition back to the neutral mode
    TransitionToNeutral,
    /// Indicate a switch to Move; the last bool means to spin 180 at the end
    TransitionToMove(Coords, Dir, bool),
}

/// List of all the processes with helpers to run the process
#[derive(Debug)]
pub enum Processes {
    Move,
    Neutral,
    Scan,
}

/// Trait to define a process
pub trait Process {
    fn run(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult;
    fn init(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult;
}