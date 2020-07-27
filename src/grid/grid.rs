use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::Rng;
use std::collections::HashMap;

use super::coords::*;
use super::edge::EdgeType;
use crate::robot::*;
use crate::schema::*;

#[derive(Clone, Copy, Debug, Queryable, Insertable)]
#[table_name = "gridcells"]
pub struct GridCell {
    pub id: i32,
    pub q: i32,
    pub r: i32,
    pub edge0: EdgeType,
    pub edge60: EdgeType,
    pub edge120: EdgeType,
    pub edge180: EdgeType,
    pub edge240: EdgeType,
    pub edge300: EdgeType,
}

impl GridCell {
    pub fn new(id: i32, coords: &Coords) -> GridCell {
        GridCell {
            id: id,
            q: coords.q,
            r: coords.r,
            edge0: EdgeType::Wall,
            edge60: EdgeType::Wall,
            edge120: EdgeType::Wall,
            edge180: EdgeType::Wall,
            edge240: EdgeType::Wall,
            edge300: EdgeType::Wall,
        }
    }

    pub fn is_open(&self) -> bool {
        self.edge0 != EdgeType::Wall
            || self.edge60 != EdgeType::Wall
            || self.edge120 != EdgeType::Wall
            || self.edge180 != EdgeType::Wall
            || self.edge240 != EdgeType::Wall
            || self.edge300 != EdgeType::Wall
    }

    pub fn is_fully_open(&self) -> bool {
        self.edge0 != EdgeType::Wall
            && self.edge60 != EdgeType::Wall
            && self.edge120 != EdgeType::Wall
            && self.edge180 != EdgeType::Wall
            && self.edge240 != EdgeType::Wall
            && self.edge300 != EdgeType::Wall
    }

    /// Get the orientations that are walls
    pub fn get_walls(&self) -> Vec<Dir> {
        let mut walls: Vec<Dir> = Vec::new();
        for dir in Dir::get_vec() {
            if self.get_side(dir) == EdgeType::Wall {
                walls.push(dir);
            }
        }

        walls
    }

    /// Get the side of the cell based on orientation
    pub fn get_side(&self, orientation: Dir) -> EdgeType {
        match orientation {
            Dir::Orient0 => self.edge0,
            Dir::Orient60 => self.edge60,
            Dir::Orient120 => self.edge120,
            Dir::Orient180 => self.edge180,
            Dir::Orient240 => self.edge240,
            Dir::Orient300 => self.edge300,
        }
    }

    pub fn change_side(&mut self, orientation: &Dir, edge_type: EdgeType) {
        match orientation {
            Dir::Orient0 => self.edge0 = edge_type,
            Dir::Orient60 => self.edge60 = edge_type,
            Dir::Orient120 => self.edge120 = edge_type,
            Dir::Orient180 => self.edge180 = edge_type,
            Dir::Orient240 => self.edge240 = edge_type,
            Dir::Orient300 => self.edge300 = edge_type,
        };
    }
}

#[derive(Debug)]
pub struct Grid {
    pub cells: HashMap<Coords, GridCell>,
    pub robot_locs: HashMap<Coords, i64>,
    pub robot_strengths: HashMap<i64, i32>,
    pub valuables_locs: HashMap<Coords, i64>,
    less_than_guess: Option<i32>,
}

impl Grid {
    pub fn load(conn: &PgConnection) -> Result<Grid, String> {
        let results = gridcells::table.load::<GridCell>(conn);

        if let Err(reason) = results {
            return Err(format!("{}", reason));
        }

        let mut cells_map: HashMap<Coords, GridCell> = HashMap::new();
        for result in results.unwrap() {
            let coords = Coords {
                q: result.q,
                r: result.r,
            };
            cells_map.insert(coords, result);
        }

        Ok(Grid {
            cells: cells_map,
            robot_locs: HashMap::new(),
            robot_strengths: HashMap::new(),
            valuables_locs: HashMap::new(),
            less_than_guess: Some(4000),
        })
    }

    pub fn new(size: u32, conn: Option<&PgConnection>) -> Result<Grid, String> {
        if size == 0 {
            return Err(String::from("Improper grid size"));
        }

        let cells: HashMap<Coords, GridCell> = super::utils::generate_cells(size as i32);

        if let Some(conn) = conn {
            diesel::delete(robots::table)
                .execute(conn)
                .expect("Could not drop gridcells table");
            diesel::delete(valuables::table)
                .execute(conn)
                .expect("Could not drop gridcells table");
            diesel::delete(gridcells::table)
                .execute(conn)
                .expect("Could not drop gridcells table");
            let cell_values = cells.values().collect::<Vec<&GridCell>>();

            let mut start = 0;
            let size = cell_values.len();
            while start < size {
                let mut end = start + 300;
                if end > size {
                    end = size;
                }

                if let Some(_cells) = cell_values.get(start..end) {
                    diesel::insert_into(gridcells::table)
                        .values(_cells.to_vec())
                        .execute(conn)
                        .expect("Error saving cells");

                    start = end;
                    println!("{}/{}", start, size);
                }
            }
        }

        Ok(Grid {
            cells: cells,
            robot_locs: HashMap::new(),
            robot_strengths: HashMap::new(),
            valuables_locs: HashMap::new(),
            less_than_guess: Some(5000),
        })
    }

    pub fn get_random_open_cell(&mut self) -> Coords {
        let mut rng = rand::thread_rng();
        let mut found_coords: Option<Coords> = None;
        while let None = found_coords {
            let max_range = self.less_than_guess.unwrap_or(5000);
            let q: i32 = rng.gen_range(max_range * -1 - 1, max_range + 1);
            let r: i32 = rng.gen_range(max_range * -1 - 1, max_range + 1);

            let test_coords = Coords { q, r };

            if self.cells.contains_key(&test_coords) {
                if self.cells.get(&test_coords).unwrap().is_open() {
                    if !self.robot_locs.contains_key(&test_coords)
                        && !self.valuables_locs.contains_key(&test_coords)
                    {
                        found_coords = Some(test_coords);
                    }
                }
            } else {
                self.less_than_guess = std::cmp::max(Some(q.abs()), Some(r.abs()));
            }
        }

        let cell = self.cells.get(&found_coords.unwrap()).unwrap();
        Coords {
            q: cell.q,
            r: cell.r,
        }
    }

    /// Given a starting point, direction, field of view and distance, get the cells in this range
    pub fn get_cells(
        &self,
        start_coords: &Coords,
        dir: Dir,
        fov: i32,
        distance: i32,
    ) -> Vec<&GridCell> {
        let mut found_cells = Vec::new();

        let start_arm_dir = dir.left(fov / 2);

        // we want to sweep in arcs, moving out by radius to max distance
        for r in 1..distance + 1 {
            let mut coord = start_coords.to(&start_arm_dir, r);
            let mut scan_dir = start_arm_dir.right(60);

            if let Some(cell) = self.cells.get(&coord) {
                found_cells.push(cell);
            }

            for step in (0..fov).step_by(60) {
                scan_dir = scan_dir.right(60);

                // if we are doing a 360 scan, we don't want duplicates so...
                let mut total_steps = r + 1;
                if step == 300 {
                    total_steps = r;
                }
                for _ in 1..total_steps {
                    coord = coord.to(&scan_dir, 1);
                    if let Some(cell) = self.cells.get(&coord) {
                        found_cells.push(cell);
                    }
                }
            }
        }

        // add our current cell
        if let Some(cell) = self.cells.get(&start_coords) {
            found_cells.push(cell);
        }

        found_cells
    }

    /// Change the robot location
    pub fn update_robot_loc(&mut self, id: i64, old_coords: Coords, new_coords: Coords) {
        self.robot_locs.remove(&old_coords);
        self.robot_locs.insert(new_coords.clone(), id);
    }

    /// Get a robot id based on a location
    pub fn get_robot_id_by_loc(&self, coords: &Coords) -> Option<&i64> {
        self.robot_locs.get(coords)
    }

    /// Get the coords of a robot by id
    pub fn get_coords_by_robot_id(&self, id: &i64) -> Option<&Coords> {
        let mut coords: Option<&Coords> = None;
        for coord in self.robot_locs.keys() {
            let _id = self.robot_locs.get(coord);
            if _id.is_none() {
                continue;
            }

            if _id.unwrap() == id {
                coords = Some(coord);
            }
        }

        coords
    }

    /// Add a robot to the grid data
    pub fn add_robot(&mut self, robot: &Robot) {
        let coords = Coords {
            q: robot.data.q,
            r: robot.data.r,
        };
        let id = robot.data.id;
        let weapon_strength = weapon::WeaponModule::get_max_damage(&robot.modules.m_weapons);

        self.robot_locs.insert(coords, id);
        self.robot_strengths.insert(id, weapon_strength);
    }

    /// Remove a robot
    pub fn remove_robot_by_loc(&mut self, coords: &Coords) {
        if let Some(id) = self.robot_locs.get(coords) {
            self.robot_strengths.remove(id);
        }
        self.robot_locs.remove(coords);
    }

    /// Remove a robot by id
    pub fn remove_robot_by_id(&mut self, id: &i64) {
        self.robot_strengths.remove(id);
        let coords = self.get_coords_by_robot_id(id);

        let mut _coords: Option<Coords> = None;
        if coords.is_some() {
            let __coords = coords.unwrap();
            _coords = Some(Coords {
                q: __coords.q,
                r: __coords.r,
            });
        } else {
            return ();
        }

        self.remove_robot_by_loc(&_coords.unwrap());
    }

    /// Get a valuable id based on a location
    pub fn get_valuable_id_by_loc(&self, coords: &Coords) -> Option<&i64> {
        self.valuables_locs.get(coords)
    }

    /// remove a valuable given a location
    pub fn remove_valuable_by_loc(&mut self, coords: &Coords) {
        self.valuables_locs.remove(coords);
    }
}

#[cfg(test)]
#[test]
fn test_cell_creation() {
    let grid = Grid::new(4, None).unwrap();

    assert_eq!(61, grid.cells.len());
    assert_eq!(
        3,
        grid.get_cells(&Coords { q: 0, r: 0 }, Dir::Orient0, 0, 2)
            .len()
    );
    assert_eq!(
        6,
        grid.get_cells(&Coords { q: 0, r: 0 }, Dir::Orient0, 240, 1)
            .len()
    );
    assert_eq!(
        9,
        grid.get_cells(&Coords { q: 0, r: 0 }, Dir::Orient0, 120, 2)
            .len()
    );
    assert_eq!(
        15,
        grid.get_cells(&Coords { q: 0, r: 0 }, Dir::Orient0, 240, 2)
            .len()
    );
    assert_eq!(
        19,
        grid.get_cells(&Coords { q: 0, r: 0 }, Dir::Orient0, 360, 2)
            .len()
    );
}
