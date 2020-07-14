use diesel::PgConnection;
use std::collections::HashMap;

use super::*;
use super::ProcessResult;

#[derive(Debug, PartialEq)]
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
        if let Some(queue) = &mut robot.movement_queue {
            if !queue.is_empty() {
                println!("Move queue: {:#?}", queue);
                println!("Move step: {:#?}", queue.remove(0));
            }
        }

        let mut scanned_cells: Vec<Coords> = Vec::new();
        if let ProcessResult::ScannedCells(cells) = Scan::run(conn, robot, None) {
            scanned_cells = cells;
        }
        println!("Scanned {} cells", scanned_cells.len());

        if robot.movement_queue.is_none() || robot.movement_queue.as_ref().unwrap().is_empty() {
            robot.movement_queue = None;
            return ProcessResult::TransitionToNeutral;
        } else {
            return ProcessResult::Ok;
        }
    }

    fn init(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult {
        println!("Transition to Move");

        robot.movement_queue = None;

        let robot_coords = Coords{ q: robot.data.q, r: robot.data.r };

        if let Some(proc_result) = message {
            match proc_result {
                ProcessResult::TransitionToMove(target_coords, orientation, spin) => {
                    println!("Transition to {:?}, {:?}, {:?}", &target_coords, &orientation, spin);
                    if let Some(target_coords) = target_coords {
                        if target_coords == robot_coords {
                            if orientation.is_some() {
                                robot.movement_queue = Some(Move::find_spin(robot.data.orientation, orientation.unwrap()));
                            }
                        } else {
                            robot.movement_queue = Some(Move::find_path(conn, robot, target_coords));
                        }
                    } else {
                        // TODO: we might want to spin or just change orientation
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
        starting_coords: &Coords, 
        starting_orientation: &Dir, 
        target_coords: &Coords, 
        known_cells_full: HashMap<Coords, &GridCell>
    ) -> HashMap<Coords, FromStep> {

        // frontier holds the cells we've discovered that need to be explored
        let mut frontier: Vec<CoordsAndDir> = Vec::new();
        frontier.push(CoordsAndDir{
            coords: starting_coords.clone(), 
            dir: starting_orientation.clone(),
        });

        let mut came_from: HashMap<Coords, FromStep> = HashMap::new();
        came_from.insert(starting_coords.clone(), FromStep{coords: starting_coords.clone(), dir: Dir::Orient0});

        while frontier.len() != 0 {
            let current = frontier.remove(0);
            if &current.coords == target_coords {
                break;
            }
            
            // for each edge, see if it is open and see if it is a known cell
            // and if it is, add it to the frontier
            let cell = known_cells_full.get(&current.coords).unwrap();
            for orientation in Dir::get_side_scan_iter(current.dir) {
                // if the side isn't a wall...
                if cell.get_side(orientation) != EdgeType::Wall {
                    let new_coords = current.coords.to(&orientation, 1);
                    // if we've seen this cell, don't re add
                    if let Some(_) = came_from.get(&new_coords) {
                        continue;
                    }

                    if let Some(_) = known_cells_full.get(&new_coords) {
                        frontier.push(CoordsAndDir{
                            coords: new_coords.clone(),
                            dir: orientation,
                        });
                        let from_step = FromStep {
                            coords: current.coords.clone(),
                            dir: orientation,
                        };
                        came_from.insert(new_coords, from_step);
                    }
                }
            }
        }

        came_from
    }

    /// add the steps to spin from one orientation to the other
    fn find_spin(start_orientation: Dir, end_orientation: Dir) -> Vec<MoveStep> {
        let mut steps = Vec::new();

        let a1: i16 = start_orientation.into();
        let mut a2: i16 = end_orientation.into();

        println!("a1: {}  a2: {}", a1, a2);

        a2 -= a1;
        if a2 > 180 {
            a2 -= 360;
        } else if a2 < -180{
            a2 += 360;
        }

        println!("a1: 0  a2: {}", a2);
        while a2 != 0 {
            if a2 < 0 {
                steps.push(MoveStep::Left);
                a2 += 60;
            } else {
                steps.push(MoveStep::Right);
                a2 -= 60;
            }
        }

        steps
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
        
        // start with our target cell
        let came_from: HashMap<Coords, FromStep> = Move::flood_map(
            &starting_coords, &robot.data.orientation, &target_coords, known_cells_full
        );
        
        let mut path: Vec<&FromStep> = Vec::new();

        let mut current = match came_from.get(&target_coords) {
            Some(op_fromstep) => op_fromstep,
            None => {
                println!("Error: couldn't find the target coords in the depth map");
                return steps;
            },
        };

        while current.coords != starting_coords  {
            path.push(current);
            current = match came_from.get(&current.coords) {
                Some(op_fromstep) => op_fromstep,
                None => {
                    println!("Z");
                    return steps
                },
            };
        }

        path.push(current);
        path.reverse();

        
        println!("{:?} to {:?}", starting_coords, target_coords);
        for step in &path {
            print!("({},{}) @ {:?} -> ", step.coords.q, step.coords.r, step.dir);
        }
        println!("{}", path.len());
        // println!("{:#?}", came_from);

        steps
    }
}

#[cfg(test)]
#[test]
fn test_spins() {
    assert_eq!(
        Move::find_spin(Dir::Orient0, Dir::Orient300),
        [MoveStep::Left]
    );

    assert_eq!(
        Move::find_spin(Dir::Orient120, Dir::Orient0),
        [MoveStep::Left, MoveStep::Left]
    );

    assert_eq!(
        Move::find_spin(Dir::Orient240, Dir::Orient0),
        [MoveStep::Right, MoveStep::Right]
    );

    assert_eq!(
        Move::find_spin(Dir::Orient60, Dir::Orient240),
        [MoveStep::Right, MoveStep::Right, MoveStep::Right]
    );
}