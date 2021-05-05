#[derive(Debug, Clone, Copy)]
pub enum Player {
    X,
    O,
    NONE,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub is_taken: bool,
    pub is_taken_by: Player,
}

#[derive(Debug)]
pub struct Board {
    pub state: Vec<Point>,
}

impl Board {
    pub fn new() -> Board {
        let mut state: Vec<Point> = vec![];

        for x in 0..3 {
            for y in 0..3 {
                state.push(Point {
                    x,
                    y,
                    is_taken: false,
                    is_taken_by: Player::NONE,
                })
            }
        }

        state.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        Board { state }
    }
}
