pub mod board;
pub mod bot;

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    T,
    O
}