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
        let mut rng = rand::thread_rng();
        let e0: i32 = rng.gen_range(0,2);
        let e1: i32 = rng.gen_range(0,2);
        let e2: i32 = rng.gen_range(0,2);
        let e3: i32 = rng.gen_range(0,2);
        let e4: i32 = rng.gen_range(0,2);
        let e5: i32 = rng.gen_range(0,2);
        
        GridCell {
            id: id,
            coords: Coords{..*coords},
            edge0: e0.into(),
            edge60: e1.into(),
            edge120: e2.into(),
            edge180: e3.into(),
            edge240: e4.into(),
            edge300: e5.into(),
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
                for _ in 0..radius {
                    coords = coords.to(&dir, 1);
                    cell_count += 1;
                    let cell = GridCell::new(cell_count, &coords);
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
}

#[cfg(test)]
#[test]
fn test_cell_creation() {
    let grid = Grid::new(2).unwrap();

    assert_eq!(19, grid.cells.len());
}