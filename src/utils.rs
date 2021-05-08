use crate::board::{Board, Player};

pub fn print_board_string(board: &mut Board) {
    // Creating a new mutable string
    let mut board_string = String::new();

    // Looping through the state
    for point in board.state.iter() {
        // Checking by which player the spot is taken
        let taken_by = match point.is_taken_by {
            Some(player) => {
                if player == Player::X {
                    "X"
                } else {
                    "O"
                }
            }
            None => "_",
        };

        // Checking if the box is the last one in the row
        // And building the game board
        if point.y.eq(&2) {
            board_string.push_str(&format!("{} \n", &taken_by))
        } else {
            board_string.push_str(&format!("{} | ", &taken_by))
        }
    }

    // Printing the board
    println!("{}", &board_string);
}
