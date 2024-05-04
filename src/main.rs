fn main() {
    let mut board = Board::new();
    board.set(1, 0);
    board.print();

    println!("0,0: {:?}", board.is_set(0, 0));
    println!("0,1: {:?}", board.is_set(0, 1));
}

struct Board {
    bytes: u64,
}

impl Board {
    fn new() -> Self {
        return Board {
            bytes: 0b0,
        }
    }
    
    fn print(&self) -> () {
        println!("");
        for row in 0..8 {
            for col in 0..8 {
                print!("{}", if self.is_set(7 - row, col) { "1" } else { "0" }); 
            }
            println!("");
        }
        println!("");
    }

    fn set(&mut self, row: usize, col: usize) {
        let offset = Board::calculate_offset(row, col);
        self.bytes |= 1 << offset;
    }

    fn is_set(&self, row: usize, col: usize) -> bool {
        let offset = Board::calculate_offset(row, col);
        return self.bytes & (1 << offset) == 1;
    }

    fn calculate_offset(row: usize, col: usize) -> usize {
        return col + row;
    }
}
