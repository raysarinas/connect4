pub mod board;
pub mod bot;

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    T,
    O
}

impl Token {
    pub fn next(&mut self) {
        match self {
            Token::T => *self = Token::O,
            Token::O => *self = Token::T,
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
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