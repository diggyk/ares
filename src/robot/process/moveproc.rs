use diesel::PgConnection;
use std::collections::HashMap;

use super::*;
use super::ProcessResult;

#[derive(Debug)]
pub enum MoveStep {
    Forward,
    Left,
    Right
}

pub struct Move {}

/// The "Move" process involves a movement queue of moves
impl Process for Move {
    /// Main run of the Neutral process
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Move: run");
        // Take the next X moves based on the drive system
        
        let mut scanned_cells: Vec<Coords> = Vec::new();
        if let ProcessResult::ScannedCells(cells) = Scan::run(conn, robot, None) {
            scanned_cells = cells;
        }
        println!("Scanned {} cells", scanned_cells.len());

        ProcessResult::TransitionToNeutral
    }

    fn init(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Move");

        if let Some(proc_result) = message {
            match proc_result {
                ProcessResult::TransitionToMove(target_coords, orientation, spin) => {
                    println!("Transition to {:?}, {:?}, {:?}", &target_coords, &orientation, spin);
                    if let Some(target_coords) = target_coords {
                        println!("{:#?}", Move::find_path(conn, robot, target_coords));
                    }
                },
                _ => {
                    return ProcessResult::Fail
                },
            }
        }
        
        // clear the movement queue
        // fill the movement queue (go to coords first, then orientation, and then 180 turn if desired)
        ProcessResult::Ok
    }
}

#[derive(Debug)]
struct FromStep {
    coords: Coords,
    dir: Dir,
}

impl Move {
    /// Get a depth and directional map from starting to end coords
    fn flood_map(
        starting_coords: &Coords, target_coords: &Coords, known_cells_full: HashMap<Coords, &GridCell>
    ) -> HashMap<Coords, Option<FromStep>> {
        let mut frontier: Vec<Coords> = Vec::new();
        frontier.push(starting_coords.clone());
        let mut came_from: HashMap<Coords, Option<FromStep>> = HashMap::new();
        came_from.insert(starting_coords.clone(), None);

        while frontier.len() != 0 {
            let current = frontier.pop().unwrap();
            if &current == target_coords {
                break;
            }
            
            // for each edge, see if it is open and see if it is a known cell
            // and if it is, add it to the frontier
            let cell = known_cells_full.get(&current).unwrap();
            for orientation in Dir::get_iter() {
                // if the side isn't a wall...
                if cell.get_side(orientation) != EdgeType::Wall {
                    let new_coords = current.to(&orientation, 1);
                    // if we've seen this cell, don't re add
                    if let Some(_) = came_from.get(&new_coords) {
                        continue;
                    }

                    if let Some(_) = known_cells_full.get(&new_coords) {
                        frontier.push(new_coords.clone());
                        let from_step = FromStep {
                            coords: current.clone(),
                            dir: orientation,
                        };
                        came_from.insert(new_coords, Some(from_step));
                    }
                }
            }
        }

        came_from
    }

    fn find_path(_: &PgConnection, robot: &mut Robot, target_coords: Coords) -> Vec<MoveStep> {
        let grid = robot.grid.lock().unwrap();

        let steps = Vec::new();
        let mut known_cells_full: HashMap<Coords, &GridCell> = HashMap::new();

        // convert the RobotKnownCell into full gridcells of the known cells
        for known_cell in &robot.known_cells {
            let coords = Coords {q: known_cell.q, r: known_cell.r };
            if let Some(cell) = grid.cells.get(&coords) {
                known_cells_full.insert(coords, cell.clone());
            }
        }

        let starting_coords = Coords{q: robot.data.r, r: robot.data.r};
        let came_from: HashMap<Coords, Option<FromStep>> = Move::flood_map(
            &starting_coords, &target_coords, known_cells_full
        );

        println!("All came_from {:#?}", came_from);

        let mut path: Vec<&FromStep> = Vec::new();

        println!("Came from: {:#?}", came_from.get(&target_coords));

        let current = match came_from.get(&target_coords) {
            Some(op_fromstep) => op_fromstep,
            None => {
                println!("Error: couldn't find the target coords in the depth map");
                return steps;
            },
        };

        println!("{:#?}", current);

        let mut current = match current {
            Some(current) => current,
            None => {
                println!("Y");
                return steps
            },
        };

        while current.coords != starting_coords  {
            path.push(current);
            let next_step = match came_from.get(&current.coords) {
                Some(op_fromstep) => op_fromstep,
                None => {
                    println!("Z");
                    return steps
                },
            };

            current = match next_step {
                Some(current) => current,
                None => {
                    println!("ZZ");
                    return steps
                },
            };
        }

        println!("{:#?}", path);

        println!("{:?} to {:?}", starting_coords, target_coords);
        println!("{:#?}", came_from);

        steps
    }
}