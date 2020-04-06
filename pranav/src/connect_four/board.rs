use crate::game_board::*;
use crate::game_elements::*;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct Board {
    board: HashMap<Coord, Token>,
    size: BoardSize,
    game_over: bool,
    game_result: Option<GameResult>,
}

impl Board {
    pub fn new(rows: Dim, cols: Dim) -> impl GameBoard {
        Board {
            board: HashMap::new(),
            size: BoardSize::new(rows, cols),
            game_over: false,
            game_result: None,
        }
    }

    pub fn find_winner(&self) -> Option<Token> {
        let rows = self.size().rows();
        let cols = self.size().cols();

        for row in 0..rows {
            for col in 0..cols {
                let token_opt = self.get_token_at((row, col));

                if token_opt.is_none() {
                    continue;
                }

                let token = token_opt.unwrap();

                // Check right
                if col + 3 < cols
                    && self.token_is_at((row, col + 1), token)
                    && self.token_is_at((row, col + 2), token)
                    && self.token_is_at((row, col + 3), token)
                {
                    return Some(token.clone());
                }

                if row + 3 < rows {
                    // Check up
                    if self.token_is_at((row + 1, col), token)
                        && self.token_is_at((row + 2, col), token)
                        && self.token_is_at((row + 3, col), token)
                    {
                        return Some(token.clone());
                    }

                    // Check up and right
                    if col + 3 < cols
                        && self.token_is_at((row + 1, col + 1), token)
                        && self.token_is_at((row + 2, col + 2), token)
                        && self.token_is_at((row + 3, col + 3), token)
                    {
                        return Some(token.clone());
                    }

                    // Check up and left
                    if col - 3 >= 0
                        && self.token_is_at((row + 1, col - 1), token)
                        && self.token_is_at((row + 2, col - 2), token)
                        && self.token_is_at((row + 3, col - 3), token)
                    {
                        return Some(token.clone());
                    }
                }
            }
        }

        None
    }
}

impl GameBoard for Board {
    fn valid_token(&mut self, token: &Token) -> bool {
        super::valid_token(token)
    }

    fn check(&mut self) {
        let mut winner = self.find_winner();

        if let Some(token) = winner {
            self.game_result = Some(GameResult::Winner(token));
            self.game_over = true;
        }

        if self.is_full() {
            self.game_result = Some(GameResult::Tie);
            self.game_over = true;
        }
    }

    fn size(&self) -> &BoardSize {
        &self.size
    }

    fn board(&self) -> &HashMap<Coord, Token> {
        &self.board
    }

    fn board_mut(&mut self) -> &mut HashMap<Coord, Token> {
        &mut self.board
    }

    fn game_over(&self) -> bool {
        self.game_over
    }

    fn game_result(&self) -> GameResult {
        self.game_result.expect("Game not over yet").clone()
    }
}
