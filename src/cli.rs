use std::io;

use crate::{board::{BOARD_HEIGHT, BOARD_LENGTH}, life::LifeController};

pub struct CLIController {
    game: LifeController
}

impl CLIController {
    pub fn new() -> CLIController {
        return CLIController {
            game: LifeController::new()
        }
    }

    pub fn edit_cells(&mut self) {
        let mut x = 0;
        let mut y = 0;

        loop {
            print!("\x1B[2J");
            self.print_with_highlight(x, y);
            let inp = getch::Getch::new();
            let a = match inp.getch() {
                Ok(v) => v as char,
                Err(_) => todo!("Handle char parsing error"),
            };
            if a == 's' {
                self.game.set_cell(x, y);
            }
            else if a == 'l' && x + 1 < BOARD_LENGTH {
                x = x + 1;
            }
            else if a == 'h' && x > 0 {
                x = x - 1;
            }
            else if a == 'k' && y < BOARD_HEIGHT - 1 {
                y = y + 1;
            }
            else if a == 'j' && y > 0 {
                y = y - 1;
            }
            else if a == 'q' {
                return;
            }
        }
    }

    pub fn print_with_highlight(&self, highlight_x: usize, highlight_y: usize) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_LENGTH {
                match self.game.is_cell_alive(
                    x, 
                    BOARD_HEIGHT - 1 - y
                ) {
                    Some(v) => {
                        let char = match v {
                            true => 'O',
                            false => '.'
                        };
                        if x == highlight_x && BOARD_HEIGHT - 1 - y == highlight_y {
                            print!("\x1b[7m{}\x1b[0m", char);  
                        } else {
                            print!("{}", char);
                        }
                    },
                    None => print!("!")
                };
            }
            print!("\n");
        }
    }
}
