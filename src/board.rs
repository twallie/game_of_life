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
    
    pub fn set_with_offset(&mut self, offset: usize) {
        self.bytes |= 0b1 << offset;
    }

    pub fn print_bytes(&self) {
        println!("{:064b}", self.bytes);
        let slice1 = self.bytes & 0b11111111;
        println!("{:08b}", slice1);
    }

    pub fn print_rows(&self) {
        println!("{:064b}", self.bytes);
        println!("");
        for row in 0..BOARD_HEIGHT {
            let slice1 = (self.bytes >> (BOARD_LENGTH * row)) & 0b11111111;
            println!("{}\t{:08b}", row, slice1);
        }
    }

    pub fn set_coordinate(&mut self, x: usize, y: usize) {
        let offset = (y * 8) + x;
        self.set_with_offset(offset);
    }
}
