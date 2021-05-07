use crate::board::{Board, Player};

// Private function for evaluating the result
fn eval_winner(board: &mut Board) -> Option<Player> {
    // For storing the rows
    let mut rows: Vec<Vec<(u32, u32, bool, Player)>> = vec![];
    // Mutable row variable that'll be pushed to `rows` vector
    let mut current_row: Vec<(u32, u32, bool, Player)> = vec![];

    // Iterating through the points
    for point in board.state.iter() {
        if current_row.len() != 3 {
            // Creating a tuple
            let position = (point.x, point.y, point.is_taken, point.is_taken_by);
            // Pusing the tuple to the row vector
            current_row.push(position);
        } else {
            // Pushing the row to `rows` array
            rows.push(current_row.clone());
            // Emptying the vector
            current_row.clear();
        }
    }

    todo!()
}

// Public function for returning a boolean if the game is over
pub fn check_game_over(board: &mut Board) -> bool {
    let eval = eval_winner(board);

    match eval {
        Some(Player::X) => {
            println!("GAME OVER: WON ❌");
            true
        }
        Some(Player::O) => {
            println!("GAME OVER: YOU WIN ⭕");
            true
        }
        Some(Player::NONE) => {
            todo!()
        }
        _ => todo!(),
    }
}
