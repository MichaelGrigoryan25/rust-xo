use rand::{prelude::SliceRandom, thread_rng};

use crate::board::{Board, Player};
use std::io::stdin;

/// Move function for the user
pub fn user_make_move(board: &mut Board) {
    // Getting the position
    let mut position_string = String::new();

    loop {
        println!("Enter the number: ");
        stdin().read_line(&mut position_string).unwrap();
        // Parsing the position
        let position: usize = position_string.trim().parse().unwrap();

        // Checking if the number is in range
        if position > 8 {
            println!("Please enter a position from range 0-8!");
            position_string.clear();
        } else {
            // Getting the point
            let player_point = board.state.get_mut(position).unwrap();

            if *player_point == None {
                // TODO: Fix the bug that occurs here
                *player_point = Some(Player::O);
                // Printing the board
                println!("{}", &board);
                break;
            } else {
                println!("That box is taken, choose another one.");
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
        let mut rng = thread_rng();
        let random_player_point = board.state.choose_mut(&mut rng).unwrap();

        if *random_player_point != None {
            continue;
        } else {
            *random_player_point = Some(Player::X);
            // Info
            println!("CPU made a move");
            // Printing the board
            println!("{}", &board);
            break;
        }
    }
}
