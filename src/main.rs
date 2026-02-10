mod grid;
use grid::Grid;

fn main() {
    let grid = Grid::new(10, 10);
    Grid::display(&grid);
}
