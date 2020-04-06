mod board_size;
mod token;

pub use board_size::BoardSize;
pub use token::Token;

// dimensions of the board
pub type Dim = isize;

// a location on the board (row, col)
pub type Coord = (Dim, Dim);

use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Winner(Token),
    Tie,
}

pub enum Turn {
    First,
    Second,
}


pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Turn {
    pub fn next(&mut self) {
        match self {
            Turn::First => *self = Turn::Second,
            Turn::Second => *self = Turn::First,
        }
    }
}

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    flush();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line");

    input.trim().to_string()
}

pub fn flush() {
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
}
