use crate::game_elements::*;
use crate::game_player::GamePlayer;

pub struct Player {
    name: String,
    token: Token,
}

impl Player {
    pub fn new(name: &str, token: Token) -> impl GamePlayer {
        assert!(
            super::valid_token(&token),
            format!("Toot Otto player cannot have a {} token", token)
        );

        Self {
            name: name.to_string(),
            token: token,
        }
    }
}

impl GamePlayer for Player {
    fn name(&self) -> &str {
        &self.name
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn choose_move(&self) -> (Dim, Token) {
        let token: Token;
        let col: Dim;

        loop {
            println!("Turn: {}", self.name());
            let token_res = read_line("Token >> ").parse::<Token>();
            if token_res.is_err() {
                println!("Invalid Token");
                println!("");
                continue;
            }

            let col_res = read_line("Col >> ").parse::<Dim>();
            if col_res.is_err() {
                println!("Invalid number for col");
                println!("");
                continue;
            }

            token = token_res.unwrap();
            col = col_res.unwrap();
            break;
        }

        (col, token)
    }
}
