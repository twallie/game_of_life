mod board;
mod errors;
mod life;
mod cli;

use std::{thread, time};

use crate::life::LifeController;
use crate::cli::CLIController;

const SLEEP_LENGTH_MS: usize = 100;

fn add_glider(life: &mut LifeController, origin_x: usize, origin_y: usize) {
    life.set_cell(1 + origin_x, 2 + origin_y).unwrap();
    life.set_cell(2 + origin_x, 1 + origin_y).unwrap();
    life.set_cell(0 + origin_x, 0 + origin_y).unwrap();
    life.set_cell(1 + origin_x, 0 + origin_y).unwrap();
    life.set_cell(2 + origin_x, 0 + origin_y).unwrap();
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
    let controller = CLIController::new();
    controller.print_with_highlight(0, 0);
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
