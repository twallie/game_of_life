use crate::{
    board::{
        Board, 
        Direction::{
            Up,
            UpRight,
            Right,
            DownRight,
            Down,
            DownLeft,
            Left,
            UpLeft
        }
    }, 
    errors::OutOfBoundsError
};

pub struct Life {
    board: Board,
}

impl Life {
    pub fn new() -> Life {
        return Life {
            board: Board::new(),
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        self.board.set_coordinate(x, y)
    }

    pub fn print_cells(&self) {
        self.board.pretty_print('O', '.');
    }

    pub fn next(&mut self) {
        let mut next_board = Board::new();

        for y in 0..64 {
            for x in 0..64 {
                match self.should_cell_live(x, y) {
                    true => match next_board.set_coordinate(x, y) {
                        Ok(_) => (),
                        Err(_) => println!("WARNING! Error occured during generation"),
                    },
                    false => ()
                };
            }
        }

        self.board = next_board;
    }

    fn should_cell_live(&self, x: usize, y: usize) -> bool {
        let is_living = match self.board.is_coordinate_set(x, y) {
            Some(v) => v,
            None => return false
        };
        let num_neighbors = self.find_number_of_cell_neighbors(x, y);

        if is_living {
            if self.is_cell_underpopulated(num_neighbors) {
                return false;
            }
            
            if self.is_cell_overpopulated(num_neighbors) {
                return false;
            }
        }

        else {
            if !self.is_cell_born(num_neighbors) {
                return false;
            }
        }

        return true;
    }

    fn is_cell_underpopulated(&self, num_neighbors: usize) -> bool { 
        num_neighbors < 2
    }

    fn is_cell_overpopulated(&self, num_neighbors: usize) -> bool {
        num_neighbors > 3
    }

    fn is_cell_born(&self, num_neighbors: usize) -> bool {
        num_neighbors == 3
    }

    fn find_number_of_cell_neighbors(&self, x: usize, y: usize) -> usize {
        let mut num = 0;
        
        let check_order = [ 
            Up, 
            UpRight, 
            Right, 
            DownRight, 
            Down, 
            DownLeft, 
            Left, 
            UpLeft
        ];

        for direction in check_order {
            match self.board.is_neighbor_set(x, y, direction) {
                Some(v) => {
                    if v {
                        num = num + 1;
                    }
                },
                None => ()
            }
        }

        num
    }
}
