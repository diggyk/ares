use std::collections::HashMap;

use super::coords::Coords;
use super::coords::Dir;
use super::edge::EdgeType;

#[derive(Debug)]
pub struct GridCell {
    pub id: u32,
    pub coords: Coords,
    pub edge0: EdgeType,
    pub edge60: EdgeType,
    pub edge120: EdgeType,
    pub edge180: EdgeType,
    pub edge240: EdgeType,
    pub edge300: EdgeType,
}

impl GridCell {
    pub fn new(coords: &Coords) -> GridCell {
        GridCell {
            id: 0,
            coords: Coords{..*coords},
            edge0: EdgeType::Wall,
            edge60: EdgeType::Wall,
            edge120: EdgeType::Wall,
            edge180: EdgeType::Wall,
            edge240: EdgeType::Wall,
            edge300: EdgeType::Wall,
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
        let root_cell = GridCell::new(&root_coords);

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
                    let cell = GridCell::new(&coords);
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