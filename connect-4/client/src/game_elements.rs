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