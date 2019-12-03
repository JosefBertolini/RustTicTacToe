use crate::space::{Space};

pub struct Gameboard {
    pub board: [[Space; 3]; 3],
}

impl Gameboard {
    pub fn new() -> [[Space; 3]; 3] {
        [[Space::EMPTY; 3]; 3]
    }


    pub fn place(&mut self, player_move: Space, row: i32, col: i32) -> bool {
        if self.board[row as usize][col as usize] == Space::EMPTY {
            self.board[row as usize][col as usize] = player_move;
            return true
        }
        false
    }

    pub fn print_board(&self) {
        println!("A - {}|{}|{}", self.board[0][0].string_form(), self.board[0][1].string_form(), self.board[0][2].string_form());
        println!("B - {}|{}|{}", self.board[1][0].string_form(), self.board[1][1].string_form(), self.board[1][2].string_form());
        println!("C - {}|{}|{}", self.board[2][0].string_form(), self.board[2][1].string_form(), self.board[2][2].string_form());
        println!("    1 2 3");
    }

    pub fn check_win(&self) -> (bool, Space) {
        for row_index in 0..3 {
            if self.board[row_index][0] != Space::EMPTY && self.board[row_index][0] == self.board[row_index][1] && self.board[row_index][0] == self.board[row_index][2] {
                return (true, self.board[row_index][0])
            }
        }
        for col_index in 0..3 {
            if self.board[0][col_index] != Space::EMPTY && self.board[0][col_index] == self.board[1][col_index] && self.board[0][col_index] == self.board[2][col_index] {
                return (true, self.board[0][col_index])
            }
        }

        if self.board[0][0] != Space::EMPTY && self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] {
            return (true, self.board[0][0])
        }
        else if self.board[0][2] != Space::EMPTY && self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] {
            return (true, self.board[0][2])
        }
        (false, Space::EMPTY)
    }
}
