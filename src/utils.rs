use crate::board::Board;

pub fn print_board_string(board: &mut Board) {
    let board_string: String = format!(
        "
        {:?} | {:?} | {:?}
        {:?} | {:?} | {:?}
        {:?} | {:?} | {:?}   
        ",
        &board.state[0].is_taken_by,
        &board.state[1].is_taken_by,
        &board.state[2].is_taken_by,
        &board.state[3].is_taken_by,
        &board.state[4].is_taken_by,
        &board.state[5].is_taken_by,
        &board.state[6].is_taken_by,
        &board.state[7].is_taken_by,
        &board.state[8].is_taken_by,
    );

    println!("{}", board_string);
}

pub fn check_game_status(board: &mut Board) -> bool {
    board.state.iter().any(|point| point.is_taken == false)
}
