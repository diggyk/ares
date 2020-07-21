use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use super::modules::*;
use super::process::*;
use crate::grid::*;
use crate::schema::*;
use crate::utils;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "robots"]
pub struct NewRobot {
    pub name: String,
    pub q: i32,
    pub r: i32,
    pub orientation: Dir,
}

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "robots"]
pub struct RobotData {
    pub id: i64,
    pub name: String,

    pub owner: Option<i32>,
    pub affiliation: Option<i32>,
    pub q: i32,
    pub r: i32,
    pub orientation: Dir,
    pub power: i32,
    pub max_power: i32,
    pub recharge_rate: i32,
}

/// Represents a grid cell that is known by a robot
#[derive(Clone, Debug, Queryable, Identifiable, Insertable)]
#[table_name = "robot_known_cells"]
#[primary_key(robot_id, gridcell_id)]
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
            .filter(robot_known_cells::robot_id.eq(robot_id))
            .load::<RobotKnownCell>(conn);

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
            && self.q == other.q
            && self.r == other.q;

        does_match
    }
}

impl Eq for RobotKnownCell {}

impl Ord for RobotKnownCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.discovery_time.cmp(&other.discovery_time)
    }
}

impl PartialOrd for RobotKnownCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents the modules loaded for this robot
#[derive(Clone, Debug, Queryable, Identifiable, Insertable, PartialEq)]
#[table_name = "robot_modules"]
#[primary_key(robot_id)]
pub struct RobotModules {
    pub robot_id: i64,
    pub m_collector: String,
    pub m_drivesystem: String,
    pub m_exfilbeacon: String,
    pub m_hull: String,
    pub m_memory: String,
    pub m_power: String,
    pub m_scanner: String,
    pub m_weapons: String,
}

impl RobotModules {
    /// Create a new robot modules struct without persisting to the db
    pub fn new(
        robot_id: i64,
        modmap: Option<HashMap<String, String>>,
        conn: Option<&PgConnection>,
    ) -> RobotModules {
        let mut modules = RobotModules {
            robot_id,
            m_collector: String::from("basic"),
            m_drivesystem: String::from("basic"),
            m_exfilbeacon: String::from("basic"),
            m_hull: String::from("basic"),
            m_memory: String::from("basic"),
            m_power: String::from("basic"),
            m_scanner: String::from("basic"),
            m_weapons: String::from("basic"),
        };

        if modmap.is_some() {
            for (key, val) in modmap.unwrap().iter() {
                match key.as_str() {
                    "m_collector" => modules.m_collector = val.to_string(),
                    "m_drivesystem" => modules.m_drivesystem = val.to_string(),
                    "m_exfilbeacon" => modules.m_exfilbeacon = val.to_string(),
                    "m_hull" => modules.m_hull = val.to_string(),
                    "m_memory" => modules.m_memory = val.to_string(),
                    "m_scanner" => modules.m_scanner = val.to_string(),
                    "m_weapon" => modules.m_weapons = val.to_string(),
                    _ => continue,
                }
            }
        }

        if let Some(conn) = conn {
            if let Err(_) = diesel::insert_into(robot_modules::table)
                .values(&modules)
                .execute(conn)
            {
                println!("Error saving modules");
            }
        }

        modules
    }
}

pub struct Robot {
    pub grid: Arc<Mutex<Grid>>,
    pub data: RobotData,
    pub known_cells: Vec<RobotKnownCell>,
    pub visible_others: Vec<VisibleRobot>,
    pub visible_valuables: Vec<VisibleValuable>,

    pub active_process: Option<Processes>,
    pub movement_queue: Option<Vec<MoveStep>>,

    pub modules: RobotModules,
}

impl Robot {
    /// Load all the robots out of the database
    pub fn load_all(conn: &PgConnection, grid: Arc<Mutex<Grid>>) -> HashMap<i64, Robot> {
        let mut _robots = HashMap::new();
        let results = robots::table
            .load::<RobotData>(conn)
            .expect("Failed to load robots");

        for result in results {
            let id = result.id;
            let mut robot = Robot {
                grid: grid.clone(),
                data: result,
                known_cells: RobotKnownCell::load_all(conn, id),
                visible_others: Vec::new(),
                visible_valuables: Vec::new(),
                active_process: None,
                movement_queue: None,
                modules: RobotModules::new(id, None, None),
            };

            if let Ok(known_cells) = robot_known_cells::table
                .filter(robot_known_cells::robot_id.eq(id))
                .load::<RobotKnownCell>(conn)
            {
                robot.known_cells = known_cells;
            }

            if let Ok(loaded_modules) = robot_modules::table
                .filter(robot_modules::robot_id.eq(id))
                .get_result::<RobotModules>(conn)
            {
                robot.modules = loaded_modules;
            }
            _robots.insert(id, robot);
        }

        _robots
    }

    /// Create a new robot at the specified coordinates with the specified orientation
    pub fn new(
        coords: Coords,
        orientation: Dir,
        conn: Option<&PgConnection>,
        grid: Arc<Mutex<Grid>>,
        modules: Option<HashMap<String, String>>,
    ) -> Robot {
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
                .get_result(conn)
                .expect("Error saving cells");
        } else {
            _robot = RobotData {
                id: 0,
                name: utils::random_string(8),
                owner: None,
                affiliation: None,
                q: coords.q,
                r: coords.r,
                orientation,
                power: 0,
                max_power: 0,
                recharge_rate: 0,
            }
        }

        let modules = RobotModules::new(_robot.id, modules, conn);

        let mut robot = Robot {
            grid,
            data: _robot,
            known_cells: Vec::new(),
            visible_others: Vec::new(),
            visible_valuables: Vec::new(),
            active_process: None,
            movement_queue: None,
            modules: modules,
        };

        robot.update_max_power(conn);

        robot
    }

    /// update the max power based on the power module
    pub fn update_max_power(&mut self, conn: Option<&PgConnection>) {
        let max_power = power::PowerModule::get_max_power(self.modules.m_power.as_str());
        let recharge_rate = power::PowerModule::get_recharge_rate(self.modules.m_power.as_str());

        self.data.max_power = max_power;
        self.data.recharge_rate = recharge_rate;

        if let Some(conn) = conn {
            let _ = diesel::update(robots::table.filter(robots::id.eq(self.data.id)))
                .set((
                    robots::max_power.eq(max_power),
                    robots::recharge_rate.eq(max_power),
                ))
                .execute(conn);
        }
    }

    /// print id and status text
    pub fn ident(&self) {
        println!(
            "Robot {}: ({},{}) @ {:?}",
            self.data.id, self.data.q, self.data.r, self.data.orientation
        );
    }

    /// Update the orientation on turn left
    pub fn turn_left(&mut self, conn: &PgConnection) {
        let orientation = self.data.orientation.left(60);
        self.data.orientation = orientation;

        let _ = diesel::update(robots::table.filter(robots::id.eq(self.data.id)))
            .set(robots::orientation.eq(&orientation))
            .execute(conn);
    }

    /// Update the orientation on turn right
    pub fn turn_right(&mut self, conn: &PgConnection) {
        let orientation = self.data.orientation.right(60);
        self.data.orientation = orientation;

        let _ = diesel::update(robots::table.filter(robots::id.eq(self.data.id)))
            .set(robots::orientation.eq(&orientation))
            .execute(conn);
    }

    /// Take a single step forward and return new coords
    pub fn move_forward(&mut self, conn: &PgConnection) -> Coords {
        let orientation = self.data.orientation;
        let new_coords = Coords {
            q: self.data.q,
            r: self.data.r,
        }
        .to(&orientation, 1);
        self.data.q = new_coords.q;
        self.data.r = new_coords.r;

        let _ = diesel::update(robots::table.filter(robots::id.eq(self.data.id)))
            .set((robots::q.eq(self.data.q), robots::r.eq(self.data.r)))
            .execute(conn);

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

    /// Update known cells with new scans; remove old results to match limits
    pub fn update_known_cells(&mut self, conn: &PgConnection, new_cells: Vec<RobotKnownCell>) {
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

        let query = diesel::insert_into(robot_known_cells::table)
            .values(&self.known_cells)
            .on_conflict((robot_known_cells::robot_id, robot_known_cells::gridcell_id))
            .do_update()
            .set(robot_known_cells::discovery_time.eq(SystemTime::now()))
            .execute(conn);

        if let Err(reason) = query {
            println!("Could not update known cells: {:?}", reason);
        }

        self.limit_known_cells(conn);
    }

    pub fn limit_known_cells(&mut self, conn: &PgConnection) {
        self.known_cells.sort();
        self.known_cells.reverse();

        let mem_limit = memory::MemoryModule::get_memory_size(self.modules.m_memory.as_str());
        let mut removed_cells = Vec::new();
        while self.known_cells.len() > mem_limit {
            let cell = self.known_cells.pop();
            if cell.is_some() {
                removed_cells.push(cell.unwrap());
            }
        }

        for removed_cell in removed_cells {
            let query = diesel::delete(
                robot_known_cells::table
                    .filter(robot_known_cells::robot_id.eq(removed_cell.robot_id))
                    .filter(robot_known_cells::gridcell_id.eq(removed_cell.gridcell_id)),
            )
            .execute(conn);
            if let Err(reason) = query {
                println!("Could not update known cells: {:?}", reason);
            }
        }
    }

    pub fn update_visible_others(&mut self, visible_robots: &Vec<VisibleRobot>) {
        self.visible_others = visible_robots.to_owned().to_vec();
    }

    pub fn update_visible_valuables(&mut self, visible_valuables: &Vec<VisibleValuable>) {
        self.visible_valuables = visible_valuables.to_owned().to_vec();
    }

    /// Get a map of coords to full gridcells that this robot knows about
    /// and isn't occupied by a known other robot
    pub fn get_known_traversable_cells(&self) -> HashMap<Coords, GridCell> {
        let grid = self.grid.lock().unwrap();
        let mut known_cells_full: HashMap<Coords, GridCell> = HashMap::new();
        // convert the RobotKnownCell into full gridcells of the known cells
        // we only want to find paths within our known cells
        for known_cell in &self.known_cells {
            let coords = Coords {
                q: known_cell.q,
                r: known_cell.r,
            };
            let robot_coords = Coords {
                q: self.data.q,
                r: self.data.r,
            };

            // ignore any cells with another robot standing on it
            if self
                .visible_others
                .iter()
                .any(|r| r.coords == coords && r.coords != robot_coords)
            {
                continue;
            }

            if let Some(cell) = grid.cells.get(&coords) {
                known_cells_full.insert(coords, *cell);
            }
        }

        known_cells_full
    }

    /// Check if the coords are known to be occupied
    pub fn known_occupied_coords(&self, coords: &Coords) -> bool {
        self.visible_others.iter().any(|r| r.coords == *coords)
    }

    /// Try to move a robot
    ///
    /// If moving the robot forward, 1) make sure there isn't a wall, and 2) make sure the
    /// cell isn't occupied; if this conditions fail, return a Fail
    /// Then update the robot's position or orientation and update grid's `robot_locs`
    pub fn move_robot(&mut self, conn: &PgConnection) -> ProcessResult {
        let robot_coords = &Coords {
            q: self.data.q,
            r: self.data.r,
        };
        let orientation = self.data.orientation;
        let next_step = self.get_move();

        // println!("Move: => {:?}", next_step);

        if next_step.is_none() {
            return ProcessResult::Fail;
        }

        match next_step.unwrap() {
            MoveStep::Left => {
                self.turn_left(conn);
            }
            MoveStep::Right => {
                self.turn_right(conn);
            }
            MoveStep::Forward => {
                let grid = self.grid.lock().unwrap();
                let cell = grid.cells.get(robot_coords);
                if cell.is_none() {
                    return ProcessResult::Fail;
                }

                if cell.unwrap().get_side(orientation) == EdgeType::Wall {
                    return ProcessResult::Fail;
                }

                if grid
                    .robot_locs
                    .get(&robot_coords.to(&orientation, 1))
                    .is_some()
                {
                    return ProcessResult::Fail;
                }

                drop(grid);

                let new_robot_coords = self.move_forward(conn).clone();
                let mut grid = self.grid.lock().unwrap();

                grid.update_robot_loc(self.data.id, robot_coords.clone(), new_robot_coords);
            }
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
            ProcessResult::TransitionToMove { .. } => {
                if Move::init(conn, self, Some(result)) == ProcessResult::Ok {
                    self.active_process = Some(Processes::Move);
                }
            }
            ProcessResult::TransitionToNeutral => {
                Neutral::init(conn, self, Some(result));
                self.active_process = Some(Processes::Neutral);
            }
            _ => {}
        }
    }
}
