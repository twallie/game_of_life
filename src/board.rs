const BOARD_LENGTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

pub struct Board {
    bytes: u64
}

impl Board {
    pub fn new() -> Board {
        return Board {
            bytes: 0b0
        }
    }
    
    fn set_with_offset(&mut self, offset: usize) {
        self.bytes |= 0b1 << offset;
    }

    fn is_offset_set(&self, offset: usize) -> bool {
        let mask = 0b1;
        let shifted = self.bytes >> offset;
        return (shifted & mask) != 0;
    }

    pub fn print_rows(&self) {
        for row in 0..BOARD_HEIGHT {
            let slice1 = (self.bytes >> (BOARD_LENGTH * row)) & 0b11111111;
            println!("{:08b}", slice1);
        }
    }

    pub fn is_coordinate_set(&self, x: usize, y: usize) -> bool {
        let offset = Board::calculate_coordinate_offset(x, y); 
        return self.is_offset_set(offset);
    }

    fn calculate_coordinate_offset(x: usize, y: usize) -> usize {
        let x_offset = 7 - x;
        let y_offset = 7 - y;
        return (y_offset * 8) + x_offset;
    }

    pub fn set_coordinate(&mut self, x: usize, y: usize) {
        let offset = Board::calculate_coordinate_offset(x, y);
        self.set_with_offset(offset);
    }
}
