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
                        if x == highlight_x && y == highlight_y {
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
