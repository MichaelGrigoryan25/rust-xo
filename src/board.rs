use std::fmt::Display;

// The player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

// Point on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub is_taken_by: Option<Player>,
}

// Board which acts as the game state
#[derive(Debug)]
pub struct Board {
    pub state: [Option<Point>; 9],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Creating a new mutable string
        let mut board_string = String::new();

        // Looping through the state
        for (index, point) in self.state.iter().enumerate() {
            // Checking by which player the spot is taken
            let taken_by = match point.map(|p| p.is_taken_by).flatten() {
                Some(Player::X) => "X".to_string(),
                Some(Player::O) => "O".to_string(),
                // If it's free then just using the index
                None => index.to_string(),
            };

            // Checking if the box is the last one in the row
            // And building the game board
            if matches!(index, 2 | 5 | 8) {
                board_string.push_str(&format!("{} \n", &taken_by))
            } else {
                board_string.push_str(&format!("{} | ", &taken_by))
            }
        }

        // Returning the board
        f.write_str(&board_string)
    }
}

impl Board {
    pub fn new() -> Self {
        Self { state: [None; 9] }
    }
}
