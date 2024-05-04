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
    }
}
