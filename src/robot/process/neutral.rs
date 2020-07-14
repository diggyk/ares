use diesel::PgConnection;

use super::*;
use super::ProcessResult;

pub struct Neutral {}

/// The "Neutral" process is when there is no active fleeing, mining, or exploring going on
impl Process for Neutral {
    /// Main run of the Neutral process
    fn run(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult {
        let mut scanned_cells: Vec<Coords> = Vec::new();
        if let ProcessResult::ScannedCells(cells) = Scan::run(conn, robot, None) {
            scanned_cells = cells;
        }
        println!("Scanned {} cells", scanned_cells.len());

        // TODO: If Others, switch to Fight or Flight

        // TODO: If on valuables, switch to Collect

        // TODO: If spotted, Valuables, switch to Move

        // TODO: Time to exfiltrate?
        
        // TODO: Switch to hibernate?

        // With nothing else to do, see what the default move is for neutral

        Neutral::next(robot)
    }
}

impl Neutral {
    /// Decide what to do next
    fn next(robot: &Robot) -> ProcessResult {
        // find random unexplore cell
        return Neutral::goto_random_unexplored_cell(robot);
    }

    fn goto_random_unexplored_cell(robot: &Robot) -> ProcessResult {
        for cell in &robot.known_cells {

        }

        ProcessResult::Ok
    }
}