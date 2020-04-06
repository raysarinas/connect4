use crate::connect_four::Board;
use crate::game_board::GameBoard;
use crate::game_elements::*;
use crate::game_player::GamePlayer;

// pub struct Factory {
//     name: String,
// }

// pub struct FactoryBot<'a> {
//     name: String,
//     factory: &'a Factory,
// }

// impl<'a> FactoryBot<'a> {
//     pub fn new<'b>(name: &str, factory: &'a Factory) -> Self {
//         Self {
//             name: name.to_string(),
//             factory: factory,
//         }
//     }
// }

// trait Bot {
//     fn new(&self) -> Self;
//     fn name(&self) -> &str;
// }

// impl<'a> Bot for FactoryBot<'a> {
//     fn name(&self) -> &str {
//         &self.name
//     }
// }



pub struct Bot<'a> {
    name: String,
    token: Token,
    dept: usize,
    board: &'a dyn GameBoard,
}

impl<'b> Bot<'b> {
    pub fn new<T: GameBoard>(token: Token, difficulty: Difficulty, board: &'b T) -> Self {
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
            name: "Connect Four Bot".to_string(),
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
        // Amazing ai algorithm here
        unimplemented!("Connect Four AI algorithm not implemented");
        // (0, self.token())
    }
}
