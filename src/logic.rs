use crate::board::{Board, Player};

pub fn eval_winner(board: &mut Board) -> Option<Player> {
    if board.state[0] == board.state[1] && board.state[1] == board.state[2] {
        board.state[0]
    } else if board.state[0] == board.state[4] && board.state[4] == board.state[8] {
        board.state[0]
    } else if board.state[0] == board.state[3] && board.state[3] == board.state[6] {
        board.state[0]
    } else if board.state[1] == board.state[4] && board.state[4] == board.state[7] {
        board.state[1]
    } else if board.state[2] == board.state[4] && board.state[4] == board.state[6] {
        board.state[2]
    } else if board.state[2] == board.state[5] && board.state[5] == board.state[8] {
        board.state[2]
    } else if board.state[3] == board.state[4] && board.state[4] == board.state[5] {
        board.state[3]
    } else if board.state[6] == board.state[7] && board.state[7] == board.state[8] {
        board.state[6]
    } else {
        None
    }
}

/// This is used for testing the `eval_winner` function
/// The output should be `ok` for every function
#[test]
fn test_all() {
    /// Function to test if the winner is X
    fn check_winner_x() {
        let mut board = Board::new();
        board.state[0] = Some(Player::X);
        board.state[4] = Some(Player::X);
        board.state[8] = Some(Player::X);
        let winner = eval_winner(&mut board);

        // Checking equality
        assert_eq!(winner, Some(Player::X));
    }

    /// Function to test if the winner is O
    fn check_winner_o() {
        let mut board = Board::new();
        board.state[0] = Some(Player::O);
        board.state[4] = Some(Player::O);
        board.state[8] = Some(Player::O);
        let winner = eval_winner(&mut board);

        // Checking equality
        assert_eq!(winner, Some(Player::O));
    }

    /// Function to test if the winner is none
    fn check_winner_none() {
        let mut board = Board::new();
        let winner = eval_winner(&mut board);

        // Checking equality
        assert_eq!(winner, None);
    }

    // Init
    check_winner_x();
    check_winner_o();
    check_winner_none();
}
