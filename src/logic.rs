use crate::board::{Board, Player};

pub fn eval_winner(board: &mut Board) -> Option<Player> {
    if &board.state[0] == &board.state[1] && &board.state[1] == &board.state[2] {
        board.state[0]
    } else if &board.state[0] == &board.state[4] && &board.state[4] == &board.state[8] {
        board.state[0]
    } else if &board.state[0] == &board.state[3] && &board.state[3] == &board.state[6] {
        board.state[0]
    } else if &board.state[1] == &board.state[4] && &board.state[4] == &board.state[7] {
        board.state[1]
    } else if &board.state[2] == &board.state[4] && &board.state[4] == &board.state[6] {
        board.state[2]
    } else if &board.state[2] == &board.state[5] && &board.state[5] == &board.state[8] {
        board.state[2]
    } else if &board.state[3] == &board.state[4] && &board.state[4] == &board.state[5] {
        board.state[3]
    } else if &board.state[6] == &board.state[7] && &board.state[7] == &board.state[8] {
        board.state[6]
    } else {
        None
    }
}
