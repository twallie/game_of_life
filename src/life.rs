use crate::{board::Board, errors::OutOfBoundsError};

pub struct Life {
    current_board: Board,
    previous_board: Board,
}

impl Life {
    pub fn new() -> Life {
        return Life {
            current_board: Board::new(),
            previous_board: Board::new()
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        self.current_board.set_coordinate(x, y)
    }

    pub fn print_cells(&self) {
        self.current_board.print_rows();
    }
}
