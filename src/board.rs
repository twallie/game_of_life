use crate::errors::OutOfBoundsError;

const BOARD_LENGTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

pub struct Board {
    bytes: u64
}

impl Board {
    pub fn new() -> Board {
        return Board {
            bytes: 0b0
        }
    }

    pub fn is_neighbor_set(&self, x: usize, y: usize, direction: Direction) -> Option<bool> {
        match direction {
            Direction::UP => {
                self.is_coordinate_set(x, y + 1)
            },
            Direction::RIGHT => {
                self.is_coordinate_set(x + 1, y)
            },
            Direction::DOWN => {
                self.is_coordinate_set(x, y - 1)
            },
            Direction::LEFT => {
                self.is_coordinate_set(x - 1, y)
            },
        }
    }

    pub fn set_coordinate(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        let offset = match Board::calculate_coordinate_offset(x, y) {
            Ok(v) => v,
            Err(v) => return Err(v),
        };
        self.set_with_offset(offset)
    }

    pub fn unset_coordinate(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        let offset = match Board::calculate_coordinate_offset(x, y) {
            Ok(v) => v,
            Err(v) => return Err(v),
        };
        self.unset_with_offset(offset)
    }
    
    pub fn is_coordinate_set(&self, x: usize, y: usize) -> Option<bool> {
        let offset = match Board::calculate_coordinate_offset(x, y) {
            Ok(v) => v,
            Err(_) => return None,
        };

        return self.is_offset_set(offset);
    }

    pub fn print_rows(&self) {
        for row in 0..BOARD_HEIGHT {
            self.print_row(row);
        }
    }

    fn set_with_offset(&mut self, offset: usize) -> Result<(), OutOfBoundsError> {
        if offset >= 64 {
            return Err(OutOfBoundsError);
        }

        self.bytes |= 0b1 << offset;
        Ok(())
    }

    fn unset_with_offset(&mut self, offset: usize) -> Result<(), OutOfBoundsError> {
        if offset >= 64 {
            return Err(OutOfBoundsError);
        }

        self.bytes &= !(1 << offset);
        Ok(())
    }

    fn is_offset_set(&self, offset: usize) -> Option<bool> {
        if offset >= 64 {
            return None;
        }

        let mask = 0b1;
        let shifted = self.bytes >> offset;
        
        Some((shifted & mask) != 0)
    }

    fn print_row(&self, row: usize) {
        let slice1 = (self.bytes >> (BOARD_LENGTH * row)) & 0b11111111;
        println!("{:08b}", slice1);
    }

    fn calculate_coordinate_offset(x: usize, y: usize) -> Result<usize, OutOfBoundsError> {
        if Board::is_out_of_bounds(x, y) {
            return Err(OutOfBoundsError)
        }

        let x_offset = 7 - x;
        let y_offset = 7 - y;
        let offset = (y_offset * 8) + x_offset;
        Ok(offset)
    }

    fn is_out_of_bounds(x: usize, y: usize) -> bool {
        x >= BOARD_LENGTH || y >= BOARD_HEIGHT
    }
}
