use crate::board::{Board, Player};

pub fn eval_winner(board: &mut Board) -> Option<Player> {
    unimplemented!()
}

/// This is used for testing the `eval_winner` function 
#[test]
fn test() {
    let mut board = Board::new();
    board.state[0] = Some(Player::X);
    board.state[4] = Some(Player::X);
    board.state[8] = Some(Player::X);
    let winner = eval_winner(&mut board);

    // Checking equality
    // The output should be: Player::X
    assert_eq!(winner, Some(Player::X));
}
