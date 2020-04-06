mod board;
mod player;
mod bot;

pub use board::Board;
pub use player::Player;
pub use bot::Bot;

use crate::game_elements::Token;

fn valid_token(token: &Token) -> bool {
    match token {
        Token::T => true,
        Token::O => true,
        _ => false,
    }
}