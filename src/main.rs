mod board;
mod errors;
mod life;

use std::{thread, time};

use crate::life::LifeController;

fn add_glider(life: &mut LifeController, origin_x: usize, origin_y: usize) {
    life.set_cell(1 + origin_x, 7 + origin_y).unwrap();
    life.set_cell(2 + origin_x, 6 + origin_y).unwrap();
    life.set_cell(0 + origin_x, 5 + origin_y).unwrap();
    life.set_cell(1 + origin_x, 5 + origin_y).unwrap();
    life.set_cell(2 + origin_x, 5 + origin_y).unwrap();
}

fn add_bee_hive(life: &mut LifeController, x: usize, y: usize) {
    life.set_cell(1 + x, 3 + y).unwrap();
    life.set_cell(2 + x, 3 + y).unwrap();
    life.set_cell(0 + x, 2 + y).unwrap();
    life.set_cell(1 + x, 1 + y).unwrap();
    life.set_cell(2 + x, 1 + y).unwrap();
    life.set_cell(3 + x, 2 + y).unwrap();
}

fn main() {
    let mut life = LifeController::new();

    // Glider
    add_glider(&mut life, 5, 20);
    add_glider(&mut life, 15, 15);

    // Bee Hive
    add_bee_hive(&mut life, 17, 10); 

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
