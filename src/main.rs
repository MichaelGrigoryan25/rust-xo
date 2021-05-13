use ansi_term::Color;
use rust_xo::board::{Board, Player};
use rust_xo::logic::eval_winner;
use rust_xo::moves::{cpu_make_move, user_make_move};

fn main() {
    // Creating a new board
    let mut board = Board::new();

    // When the program starts
    loop {
        // CPU's move
        cpu_make_move(&mut board);

        // Checking if every position is taken
        let all_taken = &board.state.iter().any(|&player| player == None);

        // If every position is taken and the winner is `None` it will be a draw
        if !all_taken && eval_winner(&mut board) == None {
            println!("{}", &Color::Yellow.paint("Draw!"));
            break;
        } else if eval_winner(&mut board) == Some(Player::X) {
            println!("{}", &Color::Red.paint("X Wins!"));
            break;
        } else if eval_winner(&mut board) == Some(Player::O) {
            println!("{}", &Color::Blue.paint("O Wins!"));
            break;
        } else {
            // Human's move
            user_make_move(&mut board);
        }
    }
}
