use owo_colors::OwoColorize;
use std::fmt::{self};

const FIRST_CLICK_PROTECTION: usize = 1;
const MINE_PERCENTAGE: usize = 30;

#[derive(PartialEq)]
enum CellType {
    Empty,
    Mine,
}

struct Cell {
    cell_type: CellType,
    revealed: bool,
    adjacent_mines: u8,
    flagged: bool,
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    mine_count: usize,
    width: usize,
    height: usize,
}

impl Cell {
    fn new() -> Self {
        Self {
            cell_type: CellType::Empty,
            revealed: true,
            adjacent_mines: 0,
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
            CellType::Mine => write!(f, "☀"),
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
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let border = "─".repeat(2 * self.width + 1);
        writeln!(f, "╭{}╮", border)?;
        for row in &self.cells {
            write!(f, "│ ")?;
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f, "│")?;
        }
        write!(f, "╰{}╯\n", border)?;
        Ok(())
    }
}

impl Grid {
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
        let (safe_y, safe_x) = safe_spot;

        for i in 1..global_size {
            let j = rand::random_range(0..=i);

            if i == j {
                continue;
            }

            let r1 = i / self.width;
            let c1 = i % self.width;
            let r2 = j / self.width;
            let c2 = j % self.width;

            if Self::distance((safe_y, safe_y), (r1, c1)) <= FIRST_CLICK_PROTECTION
                || Self::distance((safe_y, safe_x), (r2, c2)) <= FIRST_CLICK_PROTECTION
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
    }
}
