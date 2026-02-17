mod grid;
use grid::Grid;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout};

const WIDTH: usize = 30;
const HEIGHT: usize = 20;

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(|_| {
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
        let _ = disable_raw_mode();
    }));

    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let mut sel_x = 0;
    let mut sel_y = 0;
    let mut grid = Grid::new(HEIGHT, WIDTH);

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;

        grid.set_selected(sel_x, sel_y);

        print!("{}", grid);

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Up => {
                    if sel_y > 0 {
                        sel_y -= 1;
                    }
                }
                KeyCode::Down => {
                    if sel_y < HEIGHT - 1 {
                        sel_y += 1;
                    }
                }
                KeyCode::Left => {
                    if sel_x > 0 {
                        sel_x -= 1;
                    }
                }
                KeyCode::Right => {
                    if sel_x < WIDTH - 1 {
                        sel_x += 1;
                    }
                }
                KeyCode::Char(' ') => {
                    if !grid.is_generated() {
                        grid.generate((sel_x, sel_y));
                    }
                    grid.reveal(sel_x, sel_y);
                }
                KeyCode::Char('f') => {
                    grid.toggle_flag(sel_x, sel_y);
                }
                _ => {}
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;

    Ok(())
}
