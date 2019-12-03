use crate::gameboard::{Gameboard};
use crate::space::Space;
use crate::input;

pub fn run() {
    let mut game_board = Gameboard{board: Gameboard::new()};
    let mut turn = true;
    let mut counter = 0;
    welcome_message();
    loop {
        println!("Current Board:");
        game_board.print_board();
        if turn {
            println!("Player 1, it's your move");
        } else {
            println!("Player 2, it's your move");
        }

        loop {
            let (row_index, col_index) = input::read_move();
            if turn {
                let valid = game_board.place(Space::X, row_index, col_index);
                if valid {
                    break;
                }
                println!("Sorry, that is an invaild Move");
            } else {
                let valid = game_board.place(Space::Y, row_index, col_index);
                if valid {
                    break;
                }
                println!("Sorry, that is an invaild Move");
            }
        }

        let (winner, victor) = game_board.check_win();
        if winner {
            match victor {
                Space::X => println!("Player 1, You Win!"),
                Space::Y => println!("Player 2, You Win!"),
                Space::EMPTY => (),
            }
            game_board.print_board();
            break;
        }
        turn = !turn;
        counter += 1;
        if counter == 9 {
            println!("It's a Draw!");
            game_board.print_board();
            break;
        }
    }
}

fn welcome_message() {
    println!("Welcome to Tic-Tac-Toe in RUST!");
    println!("Enter you moves in the form of row,col (e.g. 'A,1')");

}
