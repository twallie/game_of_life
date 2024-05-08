use board::Board;

mod board;
mod errors;

fn main() {
    let mut board = Board::new();

    board.set_coordinate(1,1).unwrap();
    board.set_coordinate(1,2).unwrap();
    board.set_coordinate(0,1).unwrap();

    println!("\n{}\n", board.is_coordinate_set(0,1).unwrap());
    board.print_rows();

    board.unset_coordinate(0,1).unwrap();

    println!("\n{}\n", board.is_coordinate_set(0,1).unwrap());
    board.print_rows();
}
