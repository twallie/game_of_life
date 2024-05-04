use board::Board;

mod board;

fn main() {
    let mut board = Board::new();

    board.set_coordinate(0, 0);

    board.print_bytes();
    board.print_rows();
}
