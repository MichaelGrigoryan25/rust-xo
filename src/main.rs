mod board;
use board::Board;
mod moves;
mod utils;
use moves::{cpu_make_move, user_make_move};
use utils::check_game_status;

fn main() {
    let mut board = Board::new();

    loop {
        cpu_make_move(&mut board);
        if !check_game_status(&mut board) {
            println!("GAME OVER");
            break;
        }

        user_make_move(&mut board);

        if !check_game_status(&mut board) {
            println!("GAME OVER");
            break;
        }
    }
}
