mod board;
use board::Board;
mod logic;
mod moves;
use moves::{cpu_make_move, user_make_move};

fn main() {
    // Creating a new board
    let mut board = Board::new();

    // When the program starts
    loop {
        // CPU's move
        cpu_make_move(&mut board);
        // Human's move
        user_make_move(&mut board);
    }
}
