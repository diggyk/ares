use diesel::PgConnection;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::ProcessResult;
use super::*;
use crate::grid::utils::traversal;
use crate::robot::modules::collector::*;

pub struct Neutral {}

/// The "Neutral" process is when there is no active fleeing, mining, or exploring going on
impl Process for Neutral {
    /// Main run of the Neutral process
    fn run(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        // see if it is time to exfiltrate
        let max_val_inventory =
            CollectorModule::get_collection_max(robot.modules.m_collector.as_str());
        if robot.data.val_inventory >= max_val_inventory {
            return ProcessResult::TransitionToExfiltrate;
        }

        let robot_coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };

        let mut _scanned_cells: Vec<Coords> = Vec::new();
        let mut _visible_robots: Vec<VisibleRobot> = Vec::new();
        let mut _visible_valuables: Vec<VisibleValuable> = Vec::new();

        let scan_results = Scan::run(conn, robot, None);

        if let ProcessResult::ScannedCells(scan_results) = scan_results {
            _scanned_cells = scan_results.scanned_cells;
            _visible_robots = scan_results.visible_robots;
            _visible_valuables = scan_results.visible_valuables;
        } else if scan_results == ProcessResult::OutOfPower {
            return ProcessResult::OutOfPower;
        }

        // If we just scanned a robot of stronger or unknown capabilites,
        // we want to flee
        let _threats: Vec<&VisibleRobot> = _visible_robots
            .iter()
            .filter(|r| r.threat_level != ThreatLevel::Weaker)
            .collect();

        let closest_threat_coords: Option<Coords> = Neutral::find_closest_coords(
            robot,
            _visible_robots.iter().map(|r| r.coords).collect(),
            false,
        );

        if closest_threat_coords.is_some() {
            let flee_coords = Neutral::find_farthest_coords(
                robot,
                robot
                    .get_known_unoccupied_cells()
                    .keys()
                    .map(|c| c.clone())
                    .collect(),
                true,
                closest_threat_coords,
            );

            if flee_coords.is_some() {
                return ProcessResult::TransitionToFlee(
                    flee_coords.unwrap(),
                    robot.data.orientation,
                );
            }
        }

        // filter out valuables that have a robot sitting on them
        let _visible_valuables: Vec<&VisibleValuable> = _visible_valuables
            .iter()
            .filter(|v| !_visible_robots.iter().any(|r| r.coords == v.coords))
            .collect();

        if _visible_valuables.len() > 0 {
            // find the closest reachable valuables
            let closest_coords = Neutral::find_closest_coords(
                robot,
                _visible_valuables.iter().map(|v| v.coords).collect(),
                true,
            );

            // If on valuables, switch to Collect
            if closest_coords.is_some() {
                let closest_coords = closest_coords.unwrap();
                if closest_coords == robot_coords {
                    return ProcessResult::TransitionToCollect;
                }

                return ProcessResult::TransitionToMove(closest_coords, Dir::get_random(), false);
            }
        }

        // TODO: Switch to hibernate?

        // With nothing else to do, see what the default move is for neutral

        Neutral::next(robot)
    }

    // initialize this process
    fn init(conn: &PgConnection, robot: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        println!("Robot {}: Transition to Neutral", robot.data.id);
        robot.set_status_text(Some(conn), "I'm idle.");
        ProcessResult::Ok
    }
}

impl Neutral {
    /// Decide what to do next
    fn next(robot: &Robot) -> ProcessResult {
        // find random unexplore cell
        return Neutral::goto_random_unexplored_cell(robot);
    }

    // given a list of coords, pick the one that's closest
    // If `reachable` is true, we must be able to find a path to the coords based on
    // what is in memory
    fn find_closest_coords(robot: &Robot, locs: Vec<Coords>, reachable: bool) -> Option<Coords> {
        let coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };
        let mut closest: Option<Coords> = None;
        let mut shortest_distance = 100;
        for coord in locs {
            if reachable && traversal::find_path(robot, coord).is_err() {
                continue;
            }
            let distance = coords.distance_to(&coord);
            if distance < shortest_distance {
                closest = Some(coord);
                shortest_distance = distance;
            }
        }

        closest
    }

    // given a list of coords, pick the one that's closest
    // If `reachable` is true, we must be able to find a path to the coords based on
    // what is in memory
    // If 'origin_coords' is specified, we only use the robot's memory but try to find
    // the furthest point away from the origin coords instead
    fn find_farthest_coords(
        robot: &Robot,
        locs: Vec<Coords>,
        reachable: bool,
        origin_coords: Option<Coords>,
    ) -> Option<Coords> {
        let coords: Coords;
        if origin_coords.is_none() {
            coords = Coords {
                q: robot.data.q,
                r: robot.data.r,
            };
        } else {
            coords = origin_coords.unwrap();
        }

        let mut closest: Option<Coords> = None;
        let mut shortest_distance = 100;
        for coord in locs {
            if reachable && traversal::find_path(robot, coord).is_err() {
                continue;
            }
            let distance = coords.distance_to(&coord);
            if distance < shortest_distance {
                closest = Some(coord);
                shortest_distance = distance;
            }
        }

        closest
    }

    fn goto_random_unexplored_cell(robot: &Robot) -> ProcessResult {
        // the following is useful for debugging
        // return ProcessResult::TransitionToMove(Coords{q: -2, r: -2}, Dir::Orient0, false);

        let robot_coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };

        // we will look for open walls in random order
        let mut search_order: Vec<Dir> = Dir::get_vec();
        let mut rng = thread_rng();
        search_order.shuffle(&mut rng);

        // make a list of all the coordinates we know about
        let known_cells = robot.get_known_unoccupied_cells();
        let mut known_coords: Vec<Coords> = Vec::new();
        for (coords, _) in &known_cells {
            let path = traversal::find_path(robot, coords.clone());
            if path.is_ok() {
                known_coords.push(*coords);
            }
        }

        // we will search known coords in random order
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
                    // we will test the coords adjacent to the known coords
                    let test_coords = cell_coords.to(orientation, 1);

                    // we will skip testing this if the test coords are also known
                    // or if we know the coords are occupied by a robot we know about
                    if !known_coords.contains(&test_coords)
                        && !robot.known_occupied_coords(&cell_coords)
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
