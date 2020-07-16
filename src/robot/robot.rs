use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::utils;
use crate::grid::*;
use crate::schema::*;
use super::process::*;

#[derive(Debug, Queryable, Insertable)]
#[table_name="robots"]
pub struct NewRobot {
    pub name: String,
    pub q: i32,
    pub r: i32,
    pub orientation: Dir,
}

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name="robots"]
pub struct RobotData {
    pub id: i64,
    pub name: String,

    pub owner: Option<i32>,
    pub affiliation: Option<i32>,
    pub q: i32,
    pub r: i32,
    pub orientation: Dir,
    pub gridcell: Option<i32>,
    pub components: Option<serde_json::Value>,
    pub configs: Option<serde_json::Value>,
}

/// Represents a grid cell that is known by a robot
#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name="robot_known_cells"]
pub struct RobotKnownCell {
    pub robot_id: i64,
    pub gridcell_id: i32,
    pub discovery_time: std::time::SystemTime,
    pub q: i32,
    pub r: i32,
}

impl RobotKnownCell {
    /// Load all the known grid cells for a robot out of memory
    pub fn load_all(conn: &PgConnection, robot_id: i64) -> Vec<RobotKnownCell> {
        let results = robot_known_cells::table
            .filter(robot_known_cells::robot_id
            .eq(robot_id)).load::<RobotKnownCell>(conn);

        if let Ok(cells) = results {
            cells
        } else {
            Vec::new()
        }
    }
}

impl PartialEq for RobotKnownCell {
    fn eq(&self, other: &Self) -> bool {
        let does_match = self.robot_id == other.robot_id 
            && self.gridcell_id == other.gridcell_id
            && self.q == other.q && self.r == other.q;

        does_match
    }
}

#[derive(Debug)]
pub struct Robot {
    pub grid: Arc<Mutex<Grid>>,
    pub data: RobotData,
    pub known_cells: Vec<RobotKnownCell>,
    pub active_process: Option<Processes>,
    pub movement_queue: Option<Vec<MoveStep>>,
}

impl Robot {
    /// Load all the robots out of the database
    pub fn load_all(conn: &PgConnection, grid: Arc<Mutex<Grid>>) -> HashMap<i64, Robot> {
        let mut _robots = HashMap::new();
        let results = robots::table.load::<RobotData>(conn).expect("Failed to load robots");
        
        for result in results {
            let id = result.id;
            let mut robot = Robot {
                grid: grid.clone(),
                data: result,
                known_cells: RobotKnownCell::load_all(conn, id),
                active_process: None,
                movement_queue: None,
            };

            if let Ok(known_cells) = robot_known_cells::table.filter(
                robot_known_cells::robot_id.eq(id)
            ).load::<RobotKnownCell>(conn) {
                robot.known_cells = known_cells;
            }
            _robots.insert(id, robot);
        }

        _robots
    }

    /// Create a new robot at the specified coordinates with the specified orientation
    pub fn new(coords: Coords, orientation: Dir, conn: Option<&PgConnection>, grid: Arc<Mutex<Grid>>) -> Robot {
        let new_robot = NewRobot {
            name: utils::random_string(8),
            q: coords.q,
            r: coords.r,
            orientation,
        };

        let mut _robot: RobotData;
        if let Some(conn) = conn {
            _robot = diesel::insert_into(robots::table)
                .values(new_robot)
                .get_result(conn).expect("Error saving cells");
        } else {
            _robot = RobotData {
                id: 0,
                name: utils::random_string(8),
                owner: None,
                affiliation: None,
                q: coords.q,
                r: coords.r,
                orientation,
                gridcell: None,
                components: None,
                configs: None,
            }
        }

        Robot {
            grid,
            data: _robot,
            known_cells: Vec::new(),
            active_process: None,
            movement_queue: None,
        }
    }

    /// print id and status text
    pub fn ident(&self) {
        // println!("Robot {}: ({},{}) @ {:?}", self.data.id, self.data.q, self.data.r, self.data.orientation);
    }

    /// Update the orientation on turn left
    pub fn turn_left(&mut self, conn: &PgConnection) {
        let orientation = self.data.orientation.left(60);
        self.data.orientation = orientation;

        let _ = diesel::update(
            robots::table.filter(robots::id.eq(self.data.id))
        ).set(robots::orientation.eq(&orientation)).execute(conn);
    }

    /// Update the orientation on turn right
    pub fn turn_right(&mut self, conn: &PgConnection) {
        let orientation = self.data.orientation.right(60);
        self.data.orientation = orientation;

        let _ = diesel::update(
            robots::table.filter(robots::id.eq(self.data.id))
        ).set(robots::orientation.eq(&orientation)).execute(conn);
    }

    /// Take a single step forward and return new coords
    pub fn move_forward(&mut self, conn: &PgConnection) -> Coords {
        let orientation = self.data.orientation;
        let new_coords = Coords{ q: self.data.q, r: self.data.r }.to(&orientation, 1);
        self.data.q = new_coords.q;
        self.data.r = new_coords.r;

        let _ = diesel::update(
            robots::table.filter(robots::id.eq(self.data.id))
        ).set((robots::q.eq(self.data.q), robots::r.eq(self.data.r))).execute(conn);

        new_coords
    }

    /// get the next step from the movement queue
    pub fn get_move(&mut self) -> Option<MoveStep> {
        if self.movement_queue.is_none() {
            return None;
        }

        let queue = self.movement_queue.as_mut().unwrap();
        if queue.len() == 0 {
            self.movement_queue = None;
            return None;
        }

        Some(queue.remove(0))
    }

    /// empty out the movement queue
    pub fn empty_movement_queue(&mut self) {
        self.movement_queue = None;
    }

    /// Update known cells with new scans
    pub fn update_known_cells(&mut self, new_cells: Vec<RobotKnownCell>) {
        let mut new_known_cells: Vec<RobotKnownCell> = Vec::new();
        for cell in &self.known_cells {
            let mut found = false;
            for cell2 in &new_cells {
                if cell.robot_id == cell2.robot_id && cell.gridcell_id == cell2.gridcell_id {
                    found = true;
                    continue;
                }
            }
            if !found {
                new_known_cells.push(cell.clone());
            }
        }
        for cell in new_cells {
            new_known_cells.push(cell);
        }

        self.known_cells = new_known_cells;
        // println!("Known cells: {}", self.known_cells.len());
    }

    /// Try to move a robot
    /// 
    /// If moving the robot forward, 1) make sure there isn't a wall, and 2) make sure the
    /// cell isn't occupied; if this conditions fail, return a Fail
    /// Then update the robot's position or orientation and update grid's `robot_locs` 
    pub fn move_robot(&mut self, conn: &PgConnection) -> ProcessResult {
        let robot_coords = &Coords{q: self.data.q, r: self.data.r};
        let orientation = self.data.orientation;
        let next_step = self.get_move();

        // println!("Move: => {:?}", next_step);

        if next_step.is_none() {
            return ProcessResult::Fail;
        }

        match next_step.unwrap() {
            MoveStep::Left => {
                self.turn_left(conn);
            },
            MoveStep::Right => {
                self.turn_right(conn);
            },
            MoveStep::Forward => {
                let grid = self.grid.lock().unwrap();
                let cell = grid.cells.get(robot_coords);
                if cell.is_none() {
                    return ProcessResult::Fail;
                }

                if cell.unwrap().get_side(orientation) == EdgeType::Wall {
                    return ProcessResult::Fail;
                }

                if grid.robot_locs.get(&robot_coords.to(&orientation, 1)).is_some() {
                    return ProcessResult::Fail;
                }

                drop(grid);

                let new_robot_coords = self.move_forward(conn).clone();
                let mut grid = self.grid.lock().unwrap();

                grid.update_robot_loc(self.data.id, robot_coords.clone(), new_robot_coords);
            },
        }
        ProcessResult::Ok
    }

    /// Handles a tick 
    pub fn tick(&mut self, conn: &PgConnection) {
        self.ident();
        if let None = self.active_process {
            self.active_process = Some(Processes::Neutral);
        }

        let process = self.active_process.as_ref().unwrap().clone();

        let result = match process {
            Processes::Move => Move::run(conn, self, None),
            Processes::Neutral => Neutral::run(conn, self, None),
            Processes::Scan => ProcessResult::Ok,
        };

        match result {
            ProcessResult::TransitionToMove{..} => {
                if Move::init(conn, self, Some(result)) == ProcessResult::Ok {
                    self.active_process = Some(Processes::Move);
                }
            }
            ProcessResult::TransitionToNeutral => {
                Neutral::init(conn, self, Some(result));
                self.active_process = Some(Processes::Neutral);
            }
            _ => {},
        }
    }
}