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
    largeur: usize,
    hauteur: usize,
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
    pub fn new(largeur: usize, hauteur: usize) -> Self {
        let mut cells = Vec::with_capacity(hauteur);
        for _ in 0..hauteur {
            let mut ligne = Vec::with_capacity(largeur);
            for _ in 0..largeur {
                ligne.push(Cell::new());
            }
            cells.push(ligne);
        }
        Grid { cells, largeur, hauteur, initialized: false}
    }

    pub fn display(&self) {
        print!("┌─");
        for _ in 0..self.largeur {
            print!("──");
        }
        println!("┐");
        for y in 0..self.hauteur {
            print!("│ ");
            for x in 0..self.largeur {
                let cell = &self.cells[y][x];
                if cell.is_revealed {
                    print!("{} ", cell.adjacent_mines);
                } else {
                    print!("  ");
                }
            }
            println!("│");
        }
        print!("└─");
        for _ in 0..self.largeur {
            print!("──");
        }
        println!("┘");
    }

}