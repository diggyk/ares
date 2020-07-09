use ares::grid::Grid;

fn main() {
    let grid = Grid::new(2).unwrap();
    
    println!("Cells: {}", grid.cells.len())
}
