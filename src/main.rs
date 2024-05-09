mod board;
mod errors;
mod life;

use std::{thread, time};

use crate::{life::Life, board::Board};

fn main() {
    let mut life = Life::new();

    // Glider
    life.set_cell(2, 27).unwrap();
    life.set_cell(3, 26).unwrap();
    life.set_cell(1, 25).unwrap();
    life.set_cell(2, 25).unwrap();
    life.set_cell(3, 25).unwrap();

    life.print_cells();

    let sleep_length_ms = time::Duration::from_millis(200);
    for _ in 0..9999 {
        print!("\x1B[2J");
        thread::sleep(sleep_length_ms);

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
