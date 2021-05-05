use crate::{
    board::{Board, Player},
    utils::print_board_string,
};
use std::io::stdin;

pub fn user_make_move(board: &mut Board) {
    let mut position = String::new();
    println!("Enter the position(x, y): ");
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

    // Looping through the game state
    for i in board.state.iter_mut() {
        if i.x == x && i.y == y {
            // Checking if the position is free
            if !i.is_taken {
                // If the points are valid
                if i.x == x && i.y == y {
                    // Setting taken to true
                    i.is_taken = true;
                    // Setting point owner to player X
                    i.is_taken_by = Player::O;
                
                    print_board_string(board);
                    return;
                }
            } else {
            
                println!("That position is taken, try another one!");
                return user_make_move(board);
            }
            return;
        }
    }


    println!(
        "Enter a valid position! (Your previous choice {:?})",
        (&x, &y)
    );
    return user_make_move(board);
}

pub fn cpu_make_move(board: &mut Board) {
    for i in board.state.iter_mut() {
        if !i.is_taken {
            i.is_taken = true;
            i.is_taken_by = Player::X;

        
            println!("CPU made a move to position {:?}", (&i.x, &i.y));
            break;
        }
    }


    print_board_string(board);
}
