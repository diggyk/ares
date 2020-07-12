use rand::Rng;
use std::collections::HashMap;

use super::coords::Coords;
use super::coords::Dir;
use super::edge::EdgeType;

#[derive(Debug)]
pub struct GridCell {
    pub id: i32,
    pub coords: Coords,
    pub edge0: EdgeType,
    pub edge60: EdgeType,
    pub edge120: EdgeType,
    pub edge180: EdgeType,
    pub edge240: EdgeType,
    pub edge300: EdgeType,

    pub robots: Vec<i64>,
    pub valuables: Vec<i64>,
}

impl GridCell {
    pub fn new(id: i32, coords: &Coords) -> GridCell {
        GridCell {
            id: id,
            coords: Coords{..*coords},
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
            robots: Vec::new(),
            valuables: Vec::new(),
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

impl From<postgres::Row> for GridCell {
    fn from(item: postgres::Row) -> GridCell {
        let id: i32 = item.get(0);
        let q: i32 = item.get(1);
        let r: i32 = item.get(2);
        let coords = Coords{q, r};
        let e0: i16 = item.get(3);
        let e60: i16 = item.get(4);
        let e120: i16 = item.get(5);
        let e180: i16 = item.get(6);
        let e240: i16 = item.get(7);
        let e300: i16 = item.get(8);
        let edge0: EdgeType = e0.into();
        let edge60: EdgeType = e60.into();
        let edge120: EdgeType = e120.into();
        let edge180: EdgeType = e180.into();
        let edge240: EdgeType = e240.into();
        let edge300: EdgeType = e300.into();

        GridCell {
            id, coords, edge0, edge60, edge120, edge180, edge240, edge300,
            robots: Vec::new(),
            valuables: Vec::new(),
        }
    }
}


#[derive(Debug)]
pub struct Grid {
    pub cells: HashMap<Coords, GridCell>,

    less_than_guess: Option<i32>,
}

impl Grid {
    pub fn new(size: u32) -> Result<Grid, String> {
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

        Ok(
            Grid {
                cells: cells,
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

    pub fn get_random_open_cell(&mut self) -> &GridCell {
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

        self.cells.get(&found_coords.unwrap()).unwrap()
    }
}

impl From<HashMap<Coords, GridCell>> for Grid {
    fn from(items: HashMap<Coords, GridCell>) -> Grid {
        Grid {cells: items, less_than_guess: None}
    }
}

#[cfg(test)]
#[test]
fn test_cell_creation() {
    let grid = Grid::new(2).unwrap();

    assert_eq!(19, grid.cells.len());
}