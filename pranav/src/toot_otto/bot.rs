use crate::game_elements::*;
use crate::connect_four::Board;
use crate::game_board::GameBoard;
use crate::game_player::GamePlayer;

pub struct Bot<'a> {
    name: String,
    token: Token,
    dept: usize,
    board: &'a dyn GameBoard,
}

impl<'a> Bot<'a> {
    pub fn new<T: GameBoard>(token: Token, difficulty: Difficulty, board: &'a T) -> Self {
        assert!(
            super::valid_token(&token),
            format!("Connect four bot cannot have a {} token", token)
        );

        let dept = match difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 5,
            Difficulty::Hard => 9,
        };

        Self {
            name: "Toot Otto Bot".to_string(),
            token: token,
            dept: dept,
            board: board,
        }
    }
}

impl<'a> GamePlayer for Bot<'a> {
    fn name(&self) -> &str {
        &self.name
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn choose_move(&self) -> (Dim, Token) {
        // Amazing minimax ai algorithm here
        unimplemented!("Toot Otto AI algorithm not implemented");
        // (0, self.token())
    }
}
