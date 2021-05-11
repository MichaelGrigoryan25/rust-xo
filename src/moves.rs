use rand::{prelude::SliceRandom, thread_rng};

use crate::board::{Board, Player};
use std::io::stdin;

// Move function for the user
pub fn user_make_move(board: &mut Board) {
    // Getting the position
    let mut position = String::new();
    println!("Enter the number: ");
    stdin().read_line(&mut position).unwrap();
    // Parsing the position
    let position: usize = position.trim().parse().unwrap();

    // Checking if the number is in range
    if position > 8 {
        println!("Please enter a position from range 0-8");
        user_make_move(board);
    } else {
        // // Getting the point
        let point = &mut board.state.get_mut(position);

        if let Some(Some(point)) = point {
            if point.is_taken_by == Some(Player::X) {
                println!("That position is taken, please choose another one!");
                // Start again
                user_make_move(board);
            } else {
                // Info
                println!("You chose position: {}", position);
                // Make the owner of the point
                point.is_taken_by = Some(Player::O)
            }
        }
    };

    // Printing the board
    println!("{}", &board);
}

// Move function for the CPU
pub fn cpu_make_move(board: &mut Board) {
    // Choosing a random point from the game state and cloning it
    let random_point = board.state.choose_mut(&mut thread_rng());

    if let Some(Some(random_point)) = random_point {
        if random_point.is_taken_by == Some(Player::O) {
            cpu_make_move(board);
        } else {
            random_point.is_taken_by = Some(Player::X);
        }
    }

    // Info
    println!("CPU made a move");
    // Printing the board
    println!("{}", &board);
}
