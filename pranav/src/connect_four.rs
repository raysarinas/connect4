mod board;
mod player;
mod bot;

pub use board::Board;
pub use player::Player;
pub use bot::Bot;

use crate::game_elements::Token;

// check if the token is valid of this game
fn valid_token(token: &Token) -> bool {
    match token {
        Token::R => true,
        Token::Y => true,
        _ => false,
    }
}