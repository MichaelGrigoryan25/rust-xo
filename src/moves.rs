use rand::{prelude::SliceRandom, thread_rng};

use crate::board::{Board, Player};
use std::io::stdin;

/// Move function for the user
pub fn user_make_move(board: &mut Board) {
    // Creating a new empty string
    let mut position_string = String::new();

    loop {
        println!("Enter the number: ");
        // Reading the input
        stdin()
            .read_line(&mut position_string)
            .expect("Please enter a number");
        // Parsing the position as usize
        let position: usize = position_string
            .trim()
            .parse()
            .expect("Invalid argument type");

        // Checking if the number is in range
        if position > 8 {
            println!("Please enter a position from range 0-8!");
            position_string.clear();
        } else {
            // Getting the point
            let player_point = board.state.get_mut(position).unwrap();

            if *player_point == None {
                // Occupying the place
                *player_point = Some(Player::O);
                // Printing the board
                println!("{}", &board);
                break;
            } else {
                println!("That box is taken, choose another one.");
                // Clearing the string
                position_string.clear();
                continue;
            }
        };
    }
}

/// Move function for the CPU
pub fn cpu_make_move(board: &mut Board) {
    loop {
        // Choosing a random point from the game state and cloning it
        let random_player_point = board.state.choose_mut(&mut thread_rng()).unwrap();

        // If the point is occupied
        if *random_player_point != None {
            continue;
        } else {
            // Occupying the point
            *random_player_point = Some(Player::X);
            // Info
            println!("CPU made a move");
            // Printing the board
            println!("{}", &board);
            break;
        }
    }
}
