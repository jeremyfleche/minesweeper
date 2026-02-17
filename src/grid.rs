use owo_colors::OwoColorize;
use std::fmt::{self};

const FIRST_CLICK_PROTECTION: usize = 1;
const MINE_PERCENTAGE: usize = 20;

#[derive(PartialEq)]
enum CellType {
    Empty,
    Mine,
}

struct Cell {
    cell_type: CellType,
    revealed: bool,
    adjacent_mines: u8,
    adjacent_flags: u8,
    flagged: bool,
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    mine_count: usize,
    width: usize,
    height: usize,
    selected: (usize, usize),
    is_generated: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            cell_type: CellType::Empty,
            revealed: false,
            adjacent_mines: 0,
            adjacent_flags: 0,
            flagged: false,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn format_count(mine_count: u8) -> String {
            if mine_count == 0 {
                return ".".bold().to_string();
            }

            let s = mine_count.to_string();
            match mine_count {
                1 => s.blue().bold().to_string(),
                2 => s.green().bold().to_string(),
                3 => s.red().bold().to_string(),
                4 => s.magenta().to_string(),
                5 => s.bright_red().bold().to_string(),
                6 => s.cyan().to_string(),
                7 => s.black().bold().to_string(),
                8 => s.bright_black().to_string(),
                _ => s, // impossible
            }
        }

        if !self.revealed {
            return match self.flagged {
                true => write!(f, "⚑"),
                false => write!(f, " "),
            };
        }

        match self.cell_type {
            CellType::Mine => write!(f, "*"),
            CellType::Empty => write!(f, "{}", format_count(self.adjacent_mines)),
        }
    }
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let mut ligne = Vec::with_capacity(width);
            for _ in 0..width {
                ligne.push(Cell::new());
            }
            cells.push(ligne);
        }
        let mine_count = MINE_PERCENTAGE * width * height / 100;
        Grid {
            cells,
            mine_count,
            width,
            height,
            selected: (0, 0),
            is_generated: false,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let border = "─".repeat(2 * self.width + 1);
        write!(f, "╭{}╮\r\n", border)?;

        for (y, row) in self.cells.iter().enumerate() {
            write!(f, "│ ")?;
            for (x, cell) in row.iter().enumerate() {
                if self.selected == (x, y) {
                    // On affiche en inversé pour le sélecteur
                    write!(f, "{} ", cell.black().on_bright_white())?;
                } else {
                    write!(f, "{} ", cell)?;
                }
            }
            write!(f, "│\r\n")?; // \r\n ici aussi
        }
        write!(f, "╰{}╯\r\n", border)?;
        Ok(())
    }
}

impl Grid {
    pub fn set_selected(&mut self, x: usize, y: usize) {
        self.selected = (x, y);
    }

    fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
        x1.abs_diff(x2).max(y1.abs_diff(y2))
    }

    fn place_mine(&mut self, safe_spot: (usize, usize)) {
        let mut mine_placed = 0;
        let (safe_x, safe_y) = safe_spot;
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if mine_placed >= self.mine_count {
                    return;
                }

                if Self::distance((y, x), (safe_y, safe_x)) <= FIRST_CLICK_PROTECTION {
                    continue;
                }

                cell.cell_type = CellType::Mine;
                mine_placed += 1;
            }
        }
    }

    fn shuffle(&mut self, safe_spot: (usize, usize)) {
        if self.height == 0 {
            return;
        }
        let global_size = self.width * self.height;
        let (safe_x, safe_y) = safe_spot;

        for i in 1..global_size {
            let j = rand::random_range(0..=i);

            if i == j {
                continue;
            }

            let r1 = i / self.width;
            let c1 = i % self.width;
            let r2 = j / self.width;
            let c2 = j % self.width;

            if Self::distance((safe_x, safe_y), (c1, r1)) <= FIRST_CLICK_PROTECTION
                || Self::distance((safe_x, safe_y), (c2, r2)) <= FIRST_CLICK_PROTECTION
            {
                continue;
            }

            if r1 == r2 {
                self.cells[r1].swap(c1, c2);
            } else {
                let (first, second) = if r1 < r2 {
                    let (part1, part2) = self.cells.split_at_mut(r2);
                    (&mut part1[r1], &mut part2[0])
                } else {
                    let (part1, part2) = self.cells.split_at_mut(r1);
                    (&mut part2[0], &mut part1[r2])
                };
                std::mem::swap(&mut first[c1], &mut second[c2]);
            }
        }
    }

    fn update_mine_count(&mut self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let mut mine_count = 0;
                for a in -1isize..2 {
                    for b in -1isize..2 {
                        let nr = (r as isize) + a;
                        let nc = (c as isize) + b;
                        if nr >= 0
                            && nr < self.height as isize
                            && nc >= 0
                            && nc < self.width as isize
                        {
                            if self.cells[nr as usize][nc as usize].cell_type == CellType::Mine {
                                mine_count += 1;
                            }
                        }
                    }
                }
                if self.cells[r][c].cell_type == CellType::Mine {
                    mine_count -= 1;
                }
                self.cells[r][c].adjacent_mines = mine_count;
            }
        }
    }

    pub fn generate(&mut self, safe_spot: (usize, usize)) {
        self.place_mine(safe_spot);
        self.shuffle(safe_spot);
        self.update_mine_count();
        self.is_generated = true;
    }

    pub fn is_generated(&self) -> bool {
        self.is_generated
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        if self.cells[y][x].revealed || self.cells[y][x].flagged {
            return;
        }

        self.cells[y][x].revealed = true;

        if self.cells[y][x].cell_type == CellType::Mine {
            // Game over
            return;
        }

        if self.cells[y][x].adjacent_mines == 0 {
            for a in -1isize..2 {
                for b in -1isize..2 {
                    let nr = (y as isize) + a;
                    let nc = (x as isize) + b;
                    if nr >= 0 && nr < self.height as isize && nc >= 0 && nc < self.width as isize {
                        self.reveal(nc as usize, nr as usize);
                    }
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.cells[y][x].revealed {
            return;
        }
        self.cells[y][x].flagged = !self.cells[y][x].flagged;
    }
}
