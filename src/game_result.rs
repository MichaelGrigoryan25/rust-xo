use crate::board::{Board, Player, Point};

// Private function for creating a 2D array based on the board state
fn generate_2d_row_vector(board: &mut Board) -> Vec<Vec<Point>> {
    // For storing the rows - (2D array)
    let mut rows: Vec<Vec<Point>> = vec![];
    let mut current_row: Vec<Point> = vec![];

    for point in &board.state {
        current_row.push(*point);
        if current_row.len() >= 3 {
            rows.push(current_row);
            current_row = Vec::with_capacity(3);
        }
    }

    // Returning the value
    rows
}

// Private function for evaluating the winner
fn eval_winner(rows: Vec<Vec<Point>>) -> Option<Player> {
    // Looping through each row
    for (i, row) in rows.iter().enumerate() {
        // Looping through each position in the row
        for (j, &position) in row.iter().enumerate() {
            // TODO
        }
    }

    // If the game doesn't have any winners yet
    return None;
}

// Public function for returning a boolean if the game is over
pub fn check_game_over(board: &mut Board) -> bool {
    // Generate a 2D vector of board rows
    let rows = generate_2d_row_vector(board);
    // Evaluated winner
    let evaluation = eval_winner(rows);

    // Matching the result
    match evaluation {
        Some(player) => {
            if player == Player::X {
                println!("GAME OVER. WON X!");
            } else if player == Player::O {
                println!("GAME OVER. WON O!");
            } else {
                println!("GAME OVER. DRAW!");
            }

            true
        }
        None => false,
    }
}
