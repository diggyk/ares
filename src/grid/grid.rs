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
        }
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
        }
    }
}



#[derive(Debug)]
pub struct Grid {
    pub cells: HashMap<Coords, GridCell>,
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

                    // let mut rng = rand::thread_rng();
                    // let e0: i32 = rng.gen_range(0,2);
                    // let e1: i32 = rng.gen_range(0,2);
                    // let e2: i32 = rng.gen_range(0,2);
                    // let e3: i32 = rng.gen_range(0,2);
                    // let e4: i32 = rng.gen_range(0,2);
                    // let e5: i32 = rng.gen_range(0,2);
                    // cell.edge0 = e0.into();
                    // cell.edge60 = e1.into();
                    // cell.edge120 = e2.into();
                    // cell.edge180 = e3.into();
                    // cell.edge240 = e4.into();
                    // cell.edge300 = e5.into();

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
}

impl From<HashMap<Coords, GridCell>> for Grid {
    fn from(items: HashMap<Coords, GridCell>) -> Grid {
        Grid {cells: items}
    }
}

#[cfg(test)]
#[test]
fn test_cell_creation() {
    let grid = Grid::new(2).unwrap();

    assert_eq!(19, grid.cells.len());
}