pub mod traversal;
pub use traversal::*;

use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

use super::*;

pub fn generate_cells(size: i32) -> HashMap<Coords, GridCell> {
    let mut cells: HashMap<Coords, GridCell> = HashMap::new();
    let root_coords = Coords {q: 0, r: 0};
    let mut cell_count = 0;
    let root_cell = GridCell::new(cell_count, &root_coords);

    cells.insert(Coords {q: 0, r: 0}, root_cell);

    // CREATE CELLS
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

    add_rooms(&mut cells, size);
    for _ in 0..size * 2 {
        make_path(&mut cells);
    }
    enforce_outer_walls(&mut cells, size);

    cells
}

fn add_rooms(cells: &mut HashMap<Coords, GridCell>, size: i32) {
    let mut rng = rand::thread_rng();

    let coords: Vec<Coords> = cells.keys().map(|i| i.clone()).collect();
    let max_size = size/2;
    let num_rooms = size/2;

    for _ in 0..num_rooms {
        let size = rng.gen_range(1, max_size);
        let root_coords = coords.choose(&mut rng).unwrap();
        make_room(cells, &root_coords, size);
    }
}

/// Make a room
fn make_room(cells: &mut HashMap<Coords, GridCell>, root_coords: &Coords, size: i32) {
    let mut root_cell = cells.get_mut(root_coords).unwrap();

    open_cell(&mut root_cell);

    // CREATE CELLS
    // for each radius
    for radius in 1..(size + 1) as i32 {
        // we start with the bottom left direction at radius distance
        let mut coords = root_coords.to(&super::Dir::Orient240, radius);

        for angle in (0..360).step_by(60) {
            // note we never step to angle 360 b/c that's where we started
            let dir: Dir = (angle as i32).into();
            for step in 0..radius {
                coords = coords.to(&dir, 1).clone();
                if !cells.contains_key(&coords) {
                    continue;
                }

                let mut cell = cells.get_mut(&coords).unwrap();
                if cell.is_open() {
                    continue;
                }

                open_cell(&mut cell);
                if radius == size as i32 {
                    enforce_perimeter_wall(cells, &coords, &dir, step == radius - 1);
                }
            }
        }
    }
}

/// Make a path
/// We pick a random starting point and direction
/// We open the wall in direction (and in the cell beyond it)
/// We do this for a random length and then make a random turn
/// We keep doing this until we try to move to the space outside the world or we hit an open cell
fn make_path(cells: &mut HashMap<Coords, GridCell>) {
    let mut rng = rand::thread_rng();
    let coords: Vec<Coords> = cells.keys().map(|i| i.clone()).collect();

    let mut current_coord = coords.choose(&mut rng).unwrap().clone();
    let mut current_cell = cells.get_mut(&current_coord).unwrap();
    while !current_cell.is_open() || current_cell.is_fully_open() {
        current_coord = coords.choose(&mut rng).unwrap().clone();
        current_cell = cells.get_mut(&current_coord).unwrap();
    }

    let walls = current_cell.get_walls();
    let mut current_dir = walls.choose(&mut rng).unwrap();
    let mut length = rng.gen_range(1, 10);

    let mut valid_cell = true;
    while valid_cell {
        length -= 1;
        create_edge_between_cells(cells, &current_coord, current_dir, EdgeType::Open);
        
        if length <= 0 {
            length = rng.gen_range(1, 10);
            current_dir = walls.choose(&mut rng).unwrap();
        } else {
            current_coord = current_coord.to(current_dir, 1.clone());
            if let Some(_) = cells.get(&current_coord.clone()) {
                valid_cell = true;
            } else {
                valid_cell = false;
            }
        }
    }
}


/// Make sure our outer boundry has walls
fn enforce_outer_walls(cells: &mut HashMap<Coords, GridCell>, size: i32) {
    let root_coords = Coords {q: 0, r: 0};
    let mut coords = root_coords.to(&super::Dir::Orient240, size);

    for angle in (0..360).step_by(60) {
        // note we never step to angle 360 b/c that's where we started
        let dir: Dir = (angle as i32).into();
        for step in 0..size {
            coords = coords.to(&dir, 1);
            enforce_perimeter_wall(cells, &coords, &dir, step == size - 1);
        }
    }
}

/// Create a wall for a cell and it's neighbor
fn create_edge_between_cells(cells: &mut HashMap<Coords, GridCell>, coords: &Coords, dir: &Dir, edge_type: EdgeType) {
    let cell = cells.get_mut(&coords).unwrap();
    cell.change_side(dir, edge_type);

    let neighbor_coords = coords.to(&dir, 1);
    if let Some(neighbor_cell) = cells.get_mut(&neighbor_coords) {
        neighbor_cell.change_side(&dir.get_opposite(), edge_type);
    }
}

fn enforce_perimeter_wall(cells: &mut HashMap<Coords, GridCell>, coords: &Coords, dir: &Dir, last: bool){
    let cell = cells.get_mut(&coords).unwrap();
    match dir {
        Dir::Orient0 => {
            if last { create_edge_between_cells(cells, coords, &Dir::Orient0, EdgeType::Wall) }
            create_edge_between_cells(cells, coords, &Dir::Orient240, EdgeType::Wall);
            create_edge_between_cells(cells, coords, &Dir::Orient300, EdgeType::Wall);
        },
        Dir::Orient60 => {
            if last { create_edge_between_cells(cells, coords, &Dir::Orient60, EdgeType::Wall) }
            create_edge_between_cells(cells, coords, &Dir::Orient300, EdgeType::Wall);
            create_edge_between_cells(cells, coords, &Dir::Orient0, EdgeType::Wall);
        },
        Dir::Orient120 => {
            if last { create_edge_between_cells(cells, coords, &Dir::Orient120, EdgeType::Wall) }
            create_edge_between_cells(cells, coords, &Dir::Orient0, EdgeType::Wall);
            create_edge_between_cells(cells, coords, &Dir::Orient60, EdgeType::Wall);
        },
        Dir::Orient180 => {
            if last { create_edge_between_cells(cells, coords, &Dir::Orient180, EdgeType::Wall) }
            create_edge_between_cells(cells, coords, &Dir::Orient60, EdgeType::Wall);
            create_edge_between_cells(cells, coords, &Dir::Orient120, EdgeType::Wall);
        },
        Dir::Orient240 => {
            if last { create_edge_between_cells(cells, coords, &Dir::Orient240, EdgeType::Wall) }
            create_edge_between_cells(cells, coords, &Dir::Orient120, EdgeType::Wall);
            create_edge_between_cells(cells, coords, &Dir::Orient180, EdgeType::Wall);
        },
        Dir::Orient300 => {
            if last { create_edge_between_cells(cells, coords, &Dir::Orient300, EdgeType::Wall) }
            create_edge_between_cells(cells, coords, &Dir::Orient180, EdgeType::Wall);
            create_edge_between_cells(cells, coords, &Dir::Orient240, EdgeType::Wall);
        }
    }
}

/// Open up all the cell walls
fn open_cell(cell: &mut GridCell) {
    cell.edge0 = EdgeType::Open;
    cell.edge60 = EdgeType::Open;
    cell.edge120 = EdgeType::Open;
    cell.edge180 = EdgeType::Open;
    cell.edge240 = EdgeType::Open;
    cell.edge300 = EdgeType::Open;
}