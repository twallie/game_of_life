use board::Board;

mod board;
mod errors;

fn main() {
    let mut board = Board::new();

    match board.set_coordinate(100, 100) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };

    println!("\n{}\n", board.is_coordinate_set(1, 1).unwrap());

    board.print_rows();
}
