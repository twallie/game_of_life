mod board;
mod errors;
mod life;

use std::{thread, time};

use crate::life::Life;

fn main() {
    let mut life = Life::new();

    // Glider
    life.set_cell(2, 7).unwrap();
    life.set_cell(3, 6).unwrap();
    life.set_cell(1, 5).unwrap();
    life.set_cell(2, 5).unwrap();
    life.set_cell(3, 5).unwrap();

    life.print_cells();

    for _ in 0..9999 {
        print!("\x1B[2J");
        let ten_millis = time::Duration::from_millis(700);
        thread::sleep(ten_millis);

        life.next();
        life.print_cells();
    }
}

/*
DEMO THINGS
    # Bee-Hive
    life.set_cell(2, 3).unwrap();
    life.set_cell(3, 3).unwrap();
    life.set_cell(1, 2).unwrap();
    life.set_cell(2, 1).unwrap();
    life.set_cell(3, 1).unwrap();
    life.set_cell(4, 2).unwrap();
*/
