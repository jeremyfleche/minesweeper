enum CellType {
    Empty,
    Mine
}

struct Cell {
    cell_type: CellType,
    is_revealed: bool,
    adjacent_mines: u8
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    initialized: bool
}

impl Cell {
    fn new() -> Self {
        Self {
            cell_type: CellType::Empty,
            is_revealed: true,
            adjacent_mines: 0
        }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let mut ligne = Vec::with_capacity(width);
            for _ in 0..width {
                ligne.push(Cell::new());
            }
            cells.push(ligne);
        }
        Grid { cells, width, height, initialized: false}
    }

    pub fn display(&self) {
        println!("╭{}╮", "─".repeat(2*self.width+1));
        for row in &self.cells {
            print!("│ ");
            for cell in row {
                if cell.is_revealed {
                    print!("{} ", cell.adjacent_mines);
                } else {
                    print!("  ");
                }
            }
            println!("│");
        }
        println!("╰{}╯", "─".repeat(2*self.width+1));
    }

}