// The player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
    NONE,
}

// A point on the board
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub is_taken: bool,
    pub is_taken_by: Player,
}

// Board which acts as the game state
#[derive(Debug)]
pub struct Board {
    pub state: Vec<Point>,
}

impl Board {
    // For creating a new board
    pub fn new() -> Board {
        // New state holder
        let mut state: Vec<Point> = vec![];

        // Creating coordinates
        for x in 0..3 {
            for y in 0..3 {
                // Pusing the point the the state list
                state.push(Point {
                    x,
                    y,
                    is_taken: false,
                    is_taken_by: Player::NONE,
                })
            }
        }

        // Returning the board
        Board { state }
    }
}
