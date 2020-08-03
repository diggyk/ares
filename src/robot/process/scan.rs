use rand::Rng;
use std::collections::HashMap;
use std::time::SystemTime;

use super::*;
use crate::grid::utils::traversal::is_reachable;
use crate::grid::*;
use crate::robot::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ThreatLevel {
    Unknown,
    Weaker,
    Stronger,
    Equal,
}

/// Holds information about robots visible from the last scan
#[derive(Clone, Debug, PartialEq)]
pub struct VisibleRobot {
    pub robot_id: i64,
    pub coords: Coords,
    pub threat_level: ThreatLevel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VisibleValuable {
    pub valuable_id: i64,
    pub coords: Coords,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScanResults {
    pub scanned_cells: Vec<Coords>,
    pub visible_robots: Vec<VisibleRobot>,
    pub visible_valuables: Vec<VisibleValuable>,
}

pub struct Scan {}

impl Process for Scan {
    fn run(
        conn: Option<&PgConnection>,
        robot: &mut Robot,
        _: Option<ProcessResult>,
    ) -> ProcessResult {
        let mut rng = rand::thread_rng();

        // make sure we have enough power to run the scanner
        let power_need = scanner::ScannerModule::get_power_usage(&robot.modules.m_power);
        if robot.data.power < power_need {
            return ProcessResult::OutOfPower;
        }
        robot.use_power(conn, power_need);

        let our_coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };
        let grid = robot.grid.lock().unwrap();

        let fov = scanner::ScannerModule::get_fov(&robot.modules.m_scanner);
        let range = scanner::ScannerModule::get_range(&robot.modules.m_scanner);
        let accuracy = scanner::ScannerModule::get_accuracy(&robot.modules.m_scanner);
        let weapon_strength = weapon::WeaponModule::get_max_damage(&robot.modules.m_weapons);

        let cells = grid.get_cells(&our_coords, robot.data.orientation, fov, range);
        let cells_full: HashMap<Coords, GridCell> = cells
            .iter()
            .map(|cell| {
                (
                    Coords {
                        q: cell.q,
                        r: cell.r,
                    },
                    *cell.to_owned(),
                )
            })
            .collect();

        let mut known_cells: Vec<RobotKnownCell> = Vec::new();
        let mut scanned_cells: Vec<Coords> = Vec::new();
        let mut visible_robots: Vec<VisibleRobot> = Vec::new();
        let mut visible_valuables: Vec<VisibleValuable> = Vec::new();
        for cell in cells {
            // TODO: see if this cell is visible from this starting location
            let cell_coords = Coords {
                q: cell.q,
                r: cell.r,
            };
            let distance = our_coords.distance_to(&cell_coords);
            let visible = is_reachable(&our_coords, &cell_coords, &cells_full, distance);

            // if visible (or is the location we are standing on), add it to known cells
            if distance == 0 || visible {
                known_cells.push(RobotKnownCell {
                    robot_id: robot.data.id,
                    gridcell_id: cell.id,
                    discovery_time: SystemTime::now(),
                    q: cell.q,
                    r: cell.r,
                });
                scanned_cells.push(Coords {
                    q: cell.q,
                    r: cell.r,
                });

                // also see if there is a robot in that cell
                let other_robot = grid.get_robot_id_by_loc(&Coords {
                    q: cell.q,
                    r: cell.r,
                });

                // if there is a robot on that cell (and it isn't this robot)...
                if other_robot.is_some() && *other_robot.unwrap() != robot.data.id {
                    let other_strength = grid.robot_strengths.get(other_robot.unwrap());
                    let threat_level: ThreatLevel;
                    if other_strength.is_none() {
                        threat_level = ThreatLevel::Unknown;
                    } else if rng.gen_range(0, 101) > accuracy {
                        threat_level = ThreatLevel::Unknown;
                    } else if other_strength.unwrap() > &weapon_strength {
                        threat_level = ThreatLevel::Stronger;
                    } else if other_strength.unwrap() < &weapon_strength {
                        threat_level = ThreatLevel::Weaker;
                    } else {
                        threat_level = ThreatLevel::Equal;
                    }

                    visible_robots.push(VisibleRobot {
                        robot_id: *other_robot.unwrap(),
                        coords: Coords {
                            q: cell.q,
                            r: cell.r,
                        },
                        threat_level,
                    });
                }

                // and then see if there are valuables in that cell
                let valuable = grid.get_valuable_id_by_loc(&Coords {
                    q: cell.q,
                    r: cell.r,
                });
                if valuable.is_some() {
                    visible_valuables.push(VisibleValuable {
                        valuable_id: *valuable.unwrap(),
                        coords: Coords {
                            q: cell.q,
                            r: cell.r,
                        },
                    })
                }
            }
        }

        drop(grid);
        robot.update_known_cells(conn, known_cells);
        robot.update_visible_others(&visible_robots);
        robot.update_visible_valuables(&visible_valuables);

        ProcessResult::ScannedCells(ScanResults {
            scanned_cells,
            visible_robots,
            visible_valuables,
        })
    }

    // transition
    fn init(_: Option<&PgConnection>, _: &mut Robot, _: Option<ProcessResult>) -> ProcessResult {
        ProcessResult::Ok
    }
}
