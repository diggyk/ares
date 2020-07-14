use diesel::PgConnection;
use rand::thread_rng;
use rand::seq::SliceRandom;

use super::*;
use super::ProcessResult;

pub struct Neutral {}

/// The "Neutral" process is when there is no active fleeing, mining, or exploring going on
impl Process for Neutral {
    /// Main run of the Neutral process
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Neutral: run");
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

    // initialize this process
    fn init(_: &PgConnection, _: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Neutral");
        ProcessResult::Ok
    }
}

impl Neutral {
    /// Decide what to do next
    fn next(robot: &Robot) -> ProcessResult {
        // find random unexplore cell
        return Neutral::goto_random_unexplored_cell(robot);
    }

    fn goto_random_unexplored_cell(robot: &Robot) -> ProcessResult {
        let mut search_order: Vec<Dir> = Dir::get_iter().collect();
        let mut rng = thread_rng();
        search_order.shuffle(&mut rng);

        // make a list of all the coordinates we know about
        let mut known_coords: Vec<Coords> = Vec::new();
        for known_cell in &robot.known_cells {
            known_coords.push(Coords{ q: known_cell.q, r: known_cell.r });
        }

        for cell_coords in &known_coords {
            let grid = robot.grid.lock().unwrap();
            let cell = grid.cells.get(&cell_coords);
            if let None = cell {
                continue;
            }

            // check the edges in random order; if open, see if know the cell beyond it
            for orientation in &search_order {
                if cell.unwrap().get_side(*orientation) != EdgeType::Wall {
                    let test_coords = cell_coords.to(orientation, 1);
                    if known_coords.contains(&test_coords) {
                        return ProcessResult::TransitionToMove(Some(cell_coords.clone()), Some(*orientation), false);
                    }
                }
            }
        }

        ProcessResult::Ok
    }
}