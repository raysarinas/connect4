use crate::game_elements::*;
use std::collections::HashMap;

pub trait GameBoard {
    // Bottom left is 0, 0
    fn drop(&mut self, col: Dim, token: Token) -> Result<(), &'static str> {
        if !self.valid_token(&token) {
            return Err("Token is not valid");
        }

        if col >= self.size().cols() || col < 0 {
            return Err("Column out of bounds");
        }

        match self.next_free_row(col) {
            Some(row) => {
                self.put_token_at((row, col), token);
                self.check();
                Ok(())
            }
            None => Err("Column is full"),
        }
    }

    // Given a col, check the next free row
    fn next_free_row(&self, col: Dim) -> Option<Dim> {
        for row in 0..self.size().rows() {
            if self.get_token_at((row, col)).is_none() {
                return Some(row);
            }
        }

        None
    }

    // Check if a token is at a given location on the board
    fn token_is_at(&self, coord: Coord, expected: &Token) -> bool {
        match self.get_token_at(coord) {
            Some(actual) => expected == actual,
            None => false,
        }
    }

    // Get the token at a given location on the board
    fn get_token_at(&self, coord: Coord) -> Option<&Token> {
        self.board().get(&coord)
    }

    // set the token at a given location on the board
    // assert that there were no tokens there before
    fn put_token_at(&mut self, coord: Coord, token: Token) {
        let old_value = self.board_mut().insert(coord, token);
        assert_eq!(old_value, None);
    }

    // check if the board is full by checking if there is a token 
    // at every col of the top row
    fn is_full(&self) -> bool {
        let top_row = self.size().rows() - 1;

        for col in 0..self.size().cols() {
            if self.get_token_at((top_row, col)).is_none() {
                return false;
            }
        };
        
        true
    }

    fn print(&self) {
        println!("");

        // For every row...
        for row in (0..self.size().rows()).rev() {
            print!("{} | ", row); // print the row number

            // the content of every column
            for col in 0..self.size().cols() {
                match self.get_token_at((row, col)) {
                    Some(token) => print!("{} ", token),
                    None => print!("* "),
                }
            }

            println!("| "); // And a line to show the boundary
        }

        // Draw a line below the board
        print!("    ");
        for row in 0..self.size().cols() {
            print!("— ");
        }
        println!("");

        print!("    ");
        for row in 0..self.size().cols() {
            print!("{} ", row);
        }

        println!("");
        println!("");
    }

    fn valid_token(&mut self, token: &Token) -> bool;

    // Checks the board, if someone won, game_over is set to true and game_result is set
    fn check(&mut self);

    // ---- Getters ----
    fn size(&self) -> &BoardSize;

    fn board(&self) -> &HashMap<Coord, Token>;

    fn board_mut(&mut self) -> &mut HashMap<Coord, Token>;

    fn game_over(&self) -> bool;

    fn game_result(&self) -> GameResult;
}
