use ares::grid::Grid;
use ares::db::client;

fn main() {
    let grid = Grid::new(100).unwrap();
    
    client::create_cells(&grid.cells);
    println!("Cells: {}", grid.cells.len())
}
