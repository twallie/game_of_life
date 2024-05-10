use crate::errors::OutOfBoundsError;

pub const BOARD_LENGTH: usize = 64;
pub const BOARD_HEIGHT: usize = 37;

#[derive(Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

pub struct Board {
    bytes: [u64; BOARD_HEIGHT]
}

impl Board {
    pub fn new() -> Board {
        return Board {
            bytes: [0b0; BOARD_HEIGHT]
        }
    }

    pub fn is_neighbor_set(&self, x: usize, y: usize, direction: Direction) -> Option<bool> {
        match direction {
            Direction::Up => {
                if y >= BOARD_HEIGHT - 1 {
                    return None;
                }
                self.is_coordinate_set(x, y + 1)
            },
            Direction::Right => {
                if x >= BOARD_LENGTH - 1 {
                    return None
                }
                self.is_coordinate_set(x + 1, y)
            },
            Direction::Down => {
                if y <= 0 {
                    return None;
                }
                self.is_coordinate_set(x, y - 1)
            },
            Direction::Left => {
                if x <= 0 {
                    return None;
                }
                self.is_coordinate_set(x - 1, y)
            },
            Direction::UpRight => {
                if y >= BOARD_HEIGHT - 1 || x >= BOARD_LENGTH - 1 {
                    return None;
                }
                self.is_coordinate_set(x + 1, y + 1)
            },
            Direction::DownRight => {
                if y <= 0 || x >= BOARD_LENGTH - 1 {
                    return None;
                }
                self.is_coordinate_set(x + 1, y - 1)
            },
            Direction::DownLeft => {
                if y <= 0 || x <= 0 {
                    return None;
                }
                self.is_coordinate_set(x - 1, y - 1)
            },
            Direction::UpLeft => {
                if y >= BOARD_HEIGHT - 1 || x <= 0 {
                    return None;
                }
                self.is_coordinate_set(x - 1, y + 1)
            },
        }
    }

    pub fn set_coordinate(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        if Board::is_out_of_bounds(x, y) {
           return Err(OutOfBoundsError); 
        }
        self.bytes[y] |= 0b1 << 63 - x ;
        Ok(())
    }

    pub fn is_coordinate_set(&self, x: usize, y: usize) -> Option<bool> {
        if Board::is_out_of_bounds(x, y) {
           return None; 
        }
        let row = self.bytes[y];
        let byte = (row >> 63 - x) & 0b1;
        Some(byte != 0)
    }

    pub fn pretty_print(&self, alive: char, dead: char) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_LENGTH {
                match self.is_coordinate_set(
                    x, 
                    BOARD_HEIGHT - 1 - y
                ) {
                    Some(v) => {
                        match v {
                            true => print!("{}", alive),
                            false => print!("{}", dead)
                        }
                    },
                    None => println!("!")
                };
            }
            print!("\n");
        }
    }

    fn is_out_of_bounds(x: usize, y: usize) -> bool {
        x >= BOARD_LENGTH || y >= BOARD_HEIGHT
    }
}
