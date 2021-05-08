use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    board::{Board, Player},
    utils::print_board_string,
};
use std::io::stdin;

// Move function for the user
pub fn user_make_move(board: &mut Board) {
    // Getting the position
    let mut position = String::new();
    println!("Enter the position(y, x): ");
    stdin().read_line(&mut position).unwrap();
    let position: String = position.trim().parse().unwrap();

    // Splitting the string with ","
    let mut split_position_array = position.split(',');
    // Creating two separate points based on the splitted string
    let (x, y) = (
        // Position X
        split_position_array.next().unwrap().parse().unwrap(),
        // Position Y
        split_position_array.next().unwrap().parse().unwrap(),
    );

    // Checking if there's a position like that in the board state
    let found_position = board
        .state
        .iter_mut()
        .find(|&&mut point| (point.x, point.y).eq(&(x, y)));

    // Checking the cases
    match found_position {
        // Invalid position
        None => println!(
            "Enter a valid position! (Your previous choice {:?})",
            (&x, &y)
        ),
        // Valid position
        Some(point) => {
            // If the position is not taken
            if point.is_taken_by == None {
                print_board_string(board);
                // check_game_over(board);
            } else {
                println!("That position is taken, try another one!");
                return user_make_move(board);
            }
        }
    }
}

// Move function for the CPU
pub fn cpu_make_move(board: &mut Board) {
    // Choosing a random point from the game state and cloning it
    let randomly_chosen_point = board.state.choose(&mut thread_rng()).unwrap().clone();

    // If the point is taken
    if randomly_chosen_point.is_taken_by == Some(Player::O) {
        // CPU tries again
        cpu_make_move(board);
    } else {
        // Mutable iteration
        for point in board.state.iter_mut() {
            // When we find that point
            if (&point.x, &point.y).eq(&(&randomly_chosen_point.x, &randomly_chosen_point.y)) {
                // Setting the parameters
                point.is_taken_by = Some(Player::X);
                // And printing the chosen position
                println!("CPU Chose Position: {:?}", (&point.y, &point.x));
                break;
            }
        }

        // Printing the board
        print_board_string(board);
    }
}
