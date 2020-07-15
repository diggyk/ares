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
        // Take the next X moves based on the drive system
        robot.move_robot(conn);

        let mut scanned_cells: Vec<Coords> = Vec::new();
        if let ProcessResult::ScannedCells(cells) = Scan::run(conn, robot, None) {
            scanned_cells = cells;
        }
        println!("Scanned {} cells", scanned_cells.len());

        if robot.movement_queue.is_none() {
            robot.movement_queue = None;
            return ProcessResult::TransitionToNeutral;
        } else {
            return ProcessResult::Ok;
        }
    }

    fn init(conn: &PgConnection, robot: &mut Robot, message: Option<ProcessResult>) -> ProcessResult {

        robot.movement_queue = None;

        let robot_coords = Coords{ q: robot.data.q, r: robot.data.r };

        let target_coords: Coords;
        let orientation: Dir;
        let spin: bool;

        // We have to get a message containing the process result of a process
        // that decided we must move
        if let None = message {
            return ProcessResult::Fail;
        }

        let message = message.unwrap();
        match message {
            ProcessResult::TransitionToMove(tc, o, s) => {
                target_coords = tc;
                orientation = o;
                spin = s;
                println!("Transition to {:?}, {:?}, {:?}", &target_coords, &orientation, spin);
            },
            _ => {
                return ProcessResult::Fail
            },
        }

        if target_coords == robot_coords {
            robot.movement_queue = Some(Move::find_spin(robot.data.orientation, orientation));
        } else {
            let moves = Move::find_path(conn, robot, target_coords);
            match moves {
                Ok(path_queue) => robot.movement_queue = Some(path_queue),
                Err(s) => {
                    println!("{}", s);
                    return ProcessResult::Fail;
                }
            }
        }

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

        // came_from tracks how we got to each cell; start with our starting coords
        let mut came_from: HashMap<Coords, FromStep> = HashMap::new();
        came_from.insert(starting_coords.clone(), FromStep{coords: starting_coords.clone(), dir: Dir::Orient0});

        while frontier.len() != 0 {
            // take an undiscovered cell from the frontier
            // we take from the start and not the end, in order to favor
            // the direction we are facing
            let current = frontier.remove(0);
            if &current.coords == target_coords {
                break;
            }
            
            // load the full cell information
            let cell = known_cells_full.get(&current.coords).unwrap();

            // we now inspect all the edges, starting with whatever
            // direction we came into the cell with, and then alternating
            // left and right between larger and larger angles
            for orientation in Dir::get_side_scan_iter(current.dir) {
                // if the side isn't a wall...
                if cell.get_side(orientation) != EdgeType::Wall {
                    // get coordinates for the adjacent cell
                    let new_coords = current.coords.to(&orientation, 1);
                    // if we've seen this cell, don't re add
                    if let Some(_) = came_from.get(&new_coords) {
                        continue;
                    }

                    // since we haven't seen this cell adjacent cell before,
                    // add it to the frontier, noting our current orientation
                    // also add it to the tracking of how we get to this
                    // adjacent cell
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

        a2 -= a1;
        if a2 > 180 {
            a2 -= 360;
        } else if a2 < -180{
            a2 += 360;
        }

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

    /// given our list of cells and where we transition into those cells
    /// traverse this from start to end
    fn depth_to_path(
        came_from: &HashMap<Coords, FromStep>,
        target_coords: Coords,
        starting_coords: Coords,
    ) -> Result<Vec<&FromStep>, String> {
        let mut path: Vec<&FromStep> = Vec::new();

        let mut current = match came_from.get(&target_coords) {
            Some(op_fromstep) => op_fromstep,
            None => {
                return Err(String::from("Error: couldn't find the target coords in the depth map"));
            },
        };

        while current.coords != starting_coords  {
            path.push(current);
            current = match came_from.get(&current.coords) {
                Some(op_fromstep) => op_fromstep,
                None => {
                    return Err(String::from("Error: couldn't get a step when traversing depth map"));
                },
            };
        }

        path.push(current);
        path.reverse();

        Ok(path)
    }

    /// Convert the path into movement steps; for each node we match orientation and then
    /// take a step forward
    fn path_to_moves(start: CoordsAndDir, path: &Vec<&FromStep>) -> Result<Vec<MoveStep>, String> {
        let mut moves: Vec<MoveStep> = Vec::new();
        let mut current_orientation = start.dir;

        for step in path {
            if current_orientation != step.dir {
                moves.append(&mut Move::find_spin(current_orientation, step.dir));
            }
            moves.push(MoveStep::Forward);
            current_orientation = step.dir;
        }

        println!("Path_to_moves: {:#?}", moves);

        Ok(moves)
    }

    // Given a target coordinate, find a path there using only known cells by this robot
    fn find_path(_: &PgConnection, robot: &mut Robot, target_coords: Coords) -> Result<Vec<MoveStep>, String> {
        let grid = robot.grid.lock().unwrap();

        let mut known_cells_full: HashMap<Coords, &GridCell> = HashMap::new();

        // convert the RobotKnownCell into full gridcells of the known cells
        // we only want to find paths within our known cells
        for known_cell in &robot.known_cells {
            let coords = Coords {q: known_cell.q, r: known_cell.r };
            if let Some(cell) = grid.cells.get(&coords) {
                known_cells_full.insert(coords, cell.clone());
            }
        }

        let starting_coords = Coords{q: robot.data.q, r: robot.data.r};
        
        // Get a flood map so we know how we would get to each cell
        let came_from: HashMap<Coords, FromStep> = Move::flood_map(
            &starting_coords, &robot.data.orientation, &target_coords, known_cells_full
        );
        
        // Get the path in FromStep vector
        let path = Move::depth_to_path(&came_from, target_coords.clone(), starting_coords.clone());
        if path.is_err() {
            return Err(path.err().unwrap());
        }
        let path = path.unwrap();

        let steps = Move::path_to_moves(
            CoordsAndDir {
                coords: Coords { q: robot.data.q, r: robot.data.r },
                dir: robot.data.orientation,
            }, &path
        );

        println!("{:?} to {:?}", starting_coords.clone(), target_coords.clone());
        for step in &path {
            print!("({},{}) @ {:?} -> ", step.coords.q, step.coords.r, step.dir);
        }
        println!("{}", path.len());
        println!("{:?}", steps);
        
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