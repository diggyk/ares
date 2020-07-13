use diesel::prelude::*;
use diesel::pg::PgConnection;
use rand::Rng;
use std::collections::HashMap;

use crate::schema::*;
use super::coords::Coords;
use super::coords::Dir;
use super::edge::EdgeType;

#[derive(Debug, Queryable, Insertable)]
#[table_name="gridcells"]
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
            // edge0: EdgeType::Wall,
            // edge60: EdgeType::Wall,
            // edge120: EdgeType::Wall,
            // edge180: EdgeType::Wall,
            // edge240: EdgeType::Wall,
            // edge300: EdgeType::Wall,
            edge0: EdgeType::Open,
            edge60: EdgeType::Open,
            edge120: EdgeType::Open,
            edge180: EdgeType::Open,
            edge240: EdgeType::Open,
            edge300: EdgeType::Open,
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
}

#[derive(Debug)]
pub struct Grid {
    pub cells: HashMap<Coords, GridCell>,
    pub robot_locs: HashMap<Coords, i64>,
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
            let coords = Coords {q: result.q, r: result.r};
            cells_map.insert(coords, result);
        }

        Ok(
            Grid {
                cells: cells_map,
                robot_locs: HashMap::new(),
                valuables_locs: HashMap::new(),
                less_than_guess: Some(4000),
            }
        )

    }

    pub fn new(size: u32, conn: Option<&PgConnection>) -> Result<Grid, String> {
        if size == 0 {
            return Err(String::from("Improper grid size"))
        }

        let mut cells: HashMap<Coords, GridCell> = HashMap::new();
        let root_coords = Coords {q: 0, r: 0};
        let mut cell_count = 0;
        let root_cell = GridCell::new(cell_count, &root_coords);

        cells.insert(Coords {q: 0, r: 0}, root_cell);

        // for each radius
        for radius in 1..(size + 1) as i32 {
            // we start with the bottom left direction at radius distance
            let mut coords = root_coords.to(&super::Dir::Orient240, radius);

            for angle in (0..360).step_by(60) {
                // note we never step to angle 360 b/c that's where we started
                let dir: Dir = (angle as i32).into();
                for num in 0..radius {
                    coords = coords.to(&dir, 1);
                    cell_count += 1;
                    let mut cell = GridCell::new(cell_count, &coords);

                    if radius == size as i32 {
                        Grid::enforce_wall(&mut cell, &dir, num == radius - 1);
                    }
                    cells.insert(coords.clone(), cell);
                }
            }
        }

        if let Some(conn) = conn {
            diesel::delete(robots::table).execute(conn).expect("Could not drop gridcells table");
            diesel::delete(valuables::table).execute(conn).expect("Could not drop gridcells table");
            diesel::delete(gridcells::table).execute(conn).expect("Could not drop gridcells table");
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
                        .execute(conn).expect("Error saving cells");
                
                    start = end;
                    println!("{}/{}", start, size);
                }
            }
        }

        Ok(
            Grid {
                cells: cells,
                robot_locs: HashMap::new(),
                valuables_locs: HashMap::new(),
                less_than_guess: Some(5000),
            }
        )
    }

    fn enforce_wall(cell: &mut GridCell, dir: &Dir, last: bool){
        match dir {
            Dir::Orient0 => {
                if last { cell.edge0 = EdgeType::Wall }
                cell.edge240 = EdgeType::Wall;
                cell.edge300 = EdgeType::Wall;
            },
            Dir::Orient60 => {
                if last { cell.edge60 = EdgeType::Wall }
                cell.edge300 = EdgeType::Wall;
                cell.edge0 = EdgeType::Wall;
            },
            Dir::Orient120 => {
                if last { cell.edge120 = EdgeType::Wall }
                cell.edge0 = EdgeType::Wall;
                cell.edge60 = EdgeType::Wall;
            },
            Dir::Orient180 => {
                if last { cell.edge180 = EdgeType::Wall }
                cell.edge60 = EdgeType::Wall;
                cell.edge120 = EdgeType::Wall;
            },
            Dir::Orient240 => {
                if last { cell.edge240 = EdgeType::Wall }
                cell.edge120 = EdgeType::Wall;
                cell.edge180 = EdgeType::Wall;
            },
            Dir::Orient300 => {
                if last { cell.edge300 = EdgeType::Wall }
                cell.edge180 = EdgeType::Wall;
                cell.edge240 = EdgeType::Wall;
            }
        }
    }

    pub fn get_random_open_cell(&mut self) -> Coords {
        let mut rng = rand::thread_rng();
        let mut found_coords: Option<Coords> = None;
        while let None = found_coords {
            let max_range = self.less_than_guess.unwrap_or(5000);
            let q: i32 = rng.gen_range(max_range * -1, max_range);
            let r: i32 = rng.gen_range(max_range * -1, max_range);

            let test_coords = Coords{q,r};

            if self.cells.contains_key(&test_coords) {
                if self.cells.get(&test_coords).unwrap().is_open() {
                    found_coords = Some(test_coords);
                }
            } else {
                self.less_than_guess = std::cmp::max(Some(q.abs()), Some(r.abs()));
            }
        }

        let cell = self.cells.get(&found_coords.unwrap()).unwrap();
        Coords {q: cell.q, r: cell.r}
    }
}

#[cfg(test)]
#[test]
fn test_cell_creation() {
    let grid = Grid::new(2, None).unwrap();

    assert_eq!(19, grid.cells.len());
}