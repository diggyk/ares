use diesel::PgConnection;
use serde::Serialize;

mod collect;
mod exfil;
mod explode;
mod moveproc;
mod neutral;
mod pursue;
mod scan;

pub use collect::*;
pub use exfil::*;
pub use explode::*;
pub use moveproc::*;
pub use neutral::*;
pub use pursue::*;
pub use scan::*;

use super::Robot;
use crate::grid::*;
use crate::server::*;

/// Message returned from a process run to let robot know what to do next
#[derive(Clone, Debug, PartialEq)]
pub enum ProcessResult {
    /// Ok means the process succeeded and should remain the active process
    Ok,
    /// Indicates the process failed
    Fail,
    /// Result of collecting
    Collected(i32, i32),
    /// Indicates not enough power for operation
    OutOfPower,
    /// Result of a scan
    ScannedCells(ScanResults),
    /// Request something from the server
    ServerRequest(Request),
    /// Transition to collect
    TransitionToCollect,
    /// Transition to Exfiltration
    TransitionToExfiltrate,
    /// Transition to Explode
    TransitionToExplode,
    /// Indicate a switch to Flee, which is really a switch to Move but we log it
    TransitionToFlee(Coords, Dir),
    /// Indicate a switch to Move; the last bool means to spin 180 at the end
    TransitionToMove(Coords, Dir, bool),
    /// Transition back to the neutral mode
    TransitionToNeutral,
    /// Transition to pursuit of a robot
    TransitionToPursue(i64),
}

/// List of all the processes with helpers to run the process
#[derive(Clone, Debug, Serialize)]
pub enum Processes {
    Collect,
    Exfil,
    Explode,
    Move,
    Neutral,
    Pursue,
    Scan,
}

/// Trait to define a process
pub trait Process {
    fn run(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        message: Option<ProcessResult>,
    ) -> ProcessResult;

    fn init(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        message: Option<ProcessResult>,
    ) -> ProcessResult;
}
