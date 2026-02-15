mod grid;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(20, 20);
    grid.generate((10, 10));
    print!("{}", grid);
}
