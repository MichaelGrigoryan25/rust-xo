// The player
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

// Point on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub is_taken_by: Option<Player>,
}

impl Point {
    pub fn new(x: u32, y: u32, is_taken_by: Option<Player>) -> Point {
        Point { x, y, is_taken_by }
    }
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
                // Creating a point
                let point = Point::new(x, y, None);
                // Pusing the point the the state list
                state.push(point)
            }
        }

        // Returning the board
        Board { state }
    }
}
