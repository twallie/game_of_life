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
