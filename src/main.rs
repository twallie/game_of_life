use board::Board;
use board::Direction::{UP, DOWN, LEFT, RIGHT};

mod board;
mod errors;


fn main() {
    let mut board = Board::new();

    board.set_coordinate(1,1).unwrap();
    board.set_coordinate(1,2).unwrap();
    board.set_coordinate(0,1).unwrap();

    println!("\n{}\n", board.is_coordinate_set(0,1).unwrap());
    board.print_rows();

    println!(
        "\nUP: {}\nDOWN: {}\nRIGHT: {}\nLEFT: {}",
        board.is_neighbor_set(1,1, UP).unwrap(),
        board.is_neighbor_set(1,1, DOWN).unwrap(),
        board.is_neighbor_set(1,1, RIGHT).unwrap(),
        board.is_neighbor_set(1,1, LEFT).unwrap(),
    ); 

    board.unset_coordinate(0,1).unwrap();

    println!("\n{}\n", board.is_coordinate_set(0,1).unwrap());
    board.print_rows();

    println!(
        "\nUP: {}\nDOWN: {}\nRIGHT: {}\nLEFT: {}",
        board.is_neighbor_set(1,1, UP).unwrap(),
        board.is_neighbor_set(1,1, DOWN).unwrap(),
        board.is_neighbor_set(1,1, RIGHT).unwrap(),
        board.is_neighbor_set(1,1, LEFT).unwrap(),
    ); 
}
