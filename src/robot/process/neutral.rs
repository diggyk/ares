use diesel::PgConnection;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::ProcessResult;
use super::*;
use crate::grid::utils::traversal;

pub struct Neutral {}

/// The "Neutral" process is when there is no active fleeing, mining, or exploring going on
impl Process for Neutral {
    /// Main run of the Neutral process
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        // println!("Neutral: run");
        let mut _scanned_cells: Vec<Coords> = Vec::new();
        if let ProcessResult::ScannedCells(scan_results) = Scan::run(conn, robot, None) {
            _scanned_cells = scan_results.scanned_cells;
        }
        // println!("Scanned {} cells", scanned_cells.len());

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
        // the following is useful for debugging
        // return ProcessResult::TransitionToMove(Coords{q: -2, r: -2}, Dir::Orient0, false);

        let robot_coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };

        let mut search_order: Vec<Dir> = Dir::get_vec();
        let mut rng = thread_rng();
        search_order.shuffle(&mut rng);

        // make a list of all the coordinates we know about
        let known_cells = robot.get_known_traversable_cells();
        let mut known_coords: Vec<Coords> = Vec::new();
        for (coords, _) in &known_cells {
            if traversal::is_reachable(&robot_coords, &coords, &known_cells, 100) {
                known_coords.push(*coords);
            }
        }

        known_coords.shuffle(&mut rng);
        let mut random_pick: Option<(&Coords, &Dir)> = None;
        let mut closest: Option<(&Coords, &Dir, i32)> = None;
        let mut farthest: Option<(&Coords, &Dir, i32)> = None;
        for cell_coords in &known_coords {
            let grid = robot.grid.lock().unwrap();
            let cell = grid.cells.get(&cell_coords);
            if let None = cell {
                continue;
            }

            // check the edges in random order; if open, see if we know the cell beyond it
            for orientation in &search_order {
                if cell.unwrap().get_side(*orientation) != EdgeType::Wall {
                    let test_coords = cell_coords.to(orientation, 1);
                    if !known_coords.contains(&test_coords)
                        && !robot.known_occupied_coords(&test_coords)
                    {
                        if random_pick.is_none() {
                            random_pick = Some((cell_coords, orientation));
                        }

                        let distance = cell_coords.distance_to(&robot_coords);
                        if closest.is_none() {
                            closest = Some((cell_coords, orientation, distance));
                        } else {
                            if distance < closest.unwrap().2 {
                                closest = Some((cell_coords, orientation, distance));
                            }
                        }

                        if farthest.is_none() {
                            farthest = Some((cell_coords, orientation, distance));
                        } else {
                            if distance > farthest.unwrap().2 {
                                farthest = Some((cell_coords, orientation, distance));
                            }
                        }
                    }
                }
            }
        }

        // TODO: we should do this based on preferences but for now, we pick the closest
        if closest.is_some() {
            return ProcessResult::TransitionToMove(
                *closest.unwrap().0,
                *closest.unwrap().1,
                false,
            );
        }

        // since we didn't find anything unknown, pick a random place
        return ProcessResult::TransitionToMove(
            known_coords.first().unwrap().clone(),
            *search_order.first().unwrap(),
            false,
        );
    }
}
