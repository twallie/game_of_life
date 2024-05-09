mod board;
mod errors;
mod life;

use std::{thread, time};

use crate::life::Life;

fn main() {
    let mut life = Life::new();

    life.set_cell(3, 3).unwrap();
    life.set_cell(4, 3).unwrap();
    life.set_cell(2, 3).unwrap();

    life.print_cells();

    for _ in 0..10 {
        print!("\x1B[2J");
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

        life.next();
        println!("");
        life.print_cells();
    }
}
