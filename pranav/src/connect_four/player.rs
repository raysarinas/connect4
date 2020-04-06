use crate::game_elements::*;
use crate::game_player::GamePlayer;

use std::process;

pub struct Player {
    name: String,
    token: Token,
}

impl Player {
    pub fn new(name: &str, token: Token) -> impl GamePlayer {
        assert!(
            super::valid_token(&token),
            format!("Connect four player cannot have a {} token", token)
        );

        Self {
            name: name.to_string(),
            token: token,
        }
    }

    fn read_col(&self) -> Dim {
        let mut result = None;

        while result.is_none() {
            println!("Turn: {}, Token: {}", self.name, self.token);
            let input = read_line(">> ");

            if input.is_empty() {
                process::exit(0);
            }

            match input.parse::<Dim>() {
                Ok(answer) => result = Some(answer),
                Err(_) => println!("Invalid Input"),
            }
        }

        result.unwrap()
    }
}

impl GamePlayer for Player {
    fn token(&self) -> Token {
        self.token.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn choose_move(&self) -> (Dim, Token) {
        (self.read_col(), self.token())
    }
}
