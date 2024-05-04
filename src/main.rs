use board::Board;

mod board;

fn main() {
    let mut board = Board::new();
    
    for i in 0..64 {
        if i % 2 == 1 {
            continue;
        }

        board.set_with_offset(i);
    }

    board.print_bytes();
}
