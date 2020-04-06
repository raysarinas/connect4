use crate::game_elements::*;

pub trait GamePlayer {
    fn name(&self) -> &str;

    fn token(&self) -> Token;

    fn choose_move(&self) -> (Dim, Token);
}
