mod board;
mod errors;
mod life;

use crate::life::Life;

fn main() {
    let mut life = Life::new();

    life.set_cell(3, 3).unwrap();
    life.set_cell(4, 3).unwrap();
    life.set_cell(2, 3).unwrap();

    life.print_cells();

    life.next();
    println!("");

    life.print_cells();
}
