mod board;
mod errors;
mod life;
mod cli;

use crate::cli::CLIController;

fn main() {
    let mut controller = CLIController::new();
    controller.print_with_highlight(0, 0);
    controller.edit_cells();
    controller.run();
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
