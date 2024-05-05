use board::Board;

mod board;

fn main() {
    let mut board = Board::new();

    board.set_coordinate(0, 0);
    board.set_coordinate(1, 0);
    board.set_coordinate(1, 1);

    println!("\n{}\n", board.is_coordinate_set(1, 1));

    board.print_rows();
}
