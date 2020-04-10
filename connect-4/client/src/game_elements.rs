pub mod nav_bar;

// dimensions of the board
pub type Dim = isize;

// a location on the board (row, col)
pub type Coord = (Dim, Dim);


#[derive(PartialEq, Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub enum Turn {
    First,
    Second,
}

impl Turn {
    pub fn next(&mut self) {
        match self {
            Turn::First => *self = Turn::Second,
            Turn::Second => *self = Turn::First,
        }
    }
}