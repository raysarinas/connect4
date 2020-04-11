pub mod board;
pub mod bot;

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    R,
    Y
}

impl Token {
    pub fn next(&mut self) {
        match self {
            Token::R => *self = Token::Y,
            Token::Y => *self = Token::R,
        }
    }
}