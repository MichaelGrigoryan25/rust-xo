pub mod board;
pub mod logic;
pub mod moves;

/// For testing some cases
#[cfg(test)]
mod tests {
    use crate::board::{Board, Player};
    use crate::logic::eval_winner;

    #[test]
    /// Function to test if the winner is X
    fn check_winner_x() {
        let mut board = Board::new();
        board.state[0] = Some(Player::X);
        board.state[4] = Some(Player::X);
        board.state[8] = Some(Player::X);
        let winner = eval_winner(&mut board);

        // Checking equality
        assert_eq!(winner, Some(Player::X))
    }

    #[test]
    /// Function to test if the winner is O
    fn check_winner_o() {
        let mut board = Board::new();
        board.state[0] = Some(Player::O);
        board.state[4] = Some(Player::O);
        board.state[8] = Some(Player::O);
        let winner = eval_winner(&mut board);

        // Checking equality
        assert_eq!(winner, Some(Player::O))
    }

    #[test]
    /// Function to test if the winner is none
    fn check_winner_none() {
        let mut board = Board::new();
        let winner = eval_winner(&mut board);

        // Checking equality
        assert_eq!(winner, None)
    }
}
