use board::Board;

mod board;

fn main() {
    let mut board = Board::new();

    board.set_coordinate(0, 0);
    board.set_coordinate(7, 7);
    board.print_rows();
}
