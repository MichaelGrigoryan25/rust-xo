use std::fmt::Display;

/// The player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

/// Board which acts as the game state
#[derive(Debug)]
pub struct Board {
    pub state: [Option<Player>; 9],
}

/// Implementing Display trait for board to print it normally every time we want
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Creating a new mutable string
        let mut board_string = String::new();

        // Looping through the state
        for (index, point) in self.state.iter().enumerate() {
            // Checking by which player the spot is taken
            let taken_by = match point {
                Some(Player::X) => format!("{}", ansi_term::Color::Red.paint("X")).to_string(),
                Some(Player::O) => format!("{}", ansi_term::Color::Blue.paint("O")).to_string(),
                // If it's free then just using the index
                None => {
                    format!("{}", ansi_term::Color::Green.paint(format!("{}", &index))).to_string()
                }
            };

            // Checking if the box is the last one in the row
            // And building the game board
            if matches!(index, 2 | 5 | 8) {
                board_string.push_str(&format!("{} \n", &taken_by))
            } else {
                board_string.push_str(&format!("{} | ", &taken_by))
            }
        }

        // Divider
        board_string.push_str("___________________");

        // Returning the board
        f.write_str(&board_string)
    }
}

impl Board {
    pub fn new() -> Self {
        Self { state: [None; 9] }
    }
}
