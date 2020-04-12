use crate::game_elements::*;
use crate::toot_otto::*;

use std::collections::HashMap;
use std::cmp::{max, min};

use yew::services::console::ConsoleService;

pub type GameBoard = HashMap<Coord, Token>;

const AI_MOVE_VALUE: isize = -1;
const ROWS: isize = 4;
const COLS: isize = 6;

pub struct Bot {
    token: Token,
    depth: isize,
    board: GameBoard,
    console: ConsoleService
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            token: Token::T,
            depth: 0,
            board: HashMap::new(),
            console: ConsoleService::new()
        }
    }

    pub fn get_move(&mut self, board: GameBoard, depth: isize) -> Dim {
        self.set_board(board);
        self.set_depth(depth);
        let (_, col) = self.max_state(self.depth, -isize::max_value(), isize::max_value());
        col
    }

    fn set_board(&mut self, board: GameBoard) {
        self.board = board;
    }

    fn set_depth(&mut self, depth: isize) {
        self.depth = depth;
    }

    fn token_is_at(&self, coord: Coord, expected: &Token) -> bool {
        match self.board.get(&coord) {
            Some(actual) => expected == actual,
            None => false,
        }
    }

    fn match_ai_token(&self, row: isize, col: isize) -> isize {
        if self.token_is_at((row, col), &self.token) { // AI is negative
            -1 // AI Token
        } else if self.board.get(&(row, col)).is_none() {
            0 // Blank space
        } else {
            1 // Player's Token
        }
    }

    // return true if was able to 
    fn insert_in_col(&mut self, col: isize, token: Token) -> bool {
        let top_row = ROWS - 1;

        // if top row is filled, or if col is invalid, can't modify column
        if self.board.get(&(top_row, col)).is_some() || col < 0 || col > COLS - 1 {
            return false;
        }

        // else, find next free row
        let mut row = -1;
        for i in 0..top_row {
            if self.board.get(&(i, col)).is_none() {
                row = i;
                break;
            }
        }

        if row < 0 {    // theoretically, should never be true
            return false;
        }
        
        // insert in found row
        self.board.insert((row, col), token);
        true
    }

    fn check_state(&self) -> (isize, isize) {
        let mut win_val = 0;
        let mut chain_val = 0;

        for i in 0..ROWS {
            for j in 0..COLS {
                let mut temp_r = 0;
                let mut temp_t = 0;
                let mut temp_tr = 0;
                let mut temp_br = 0;
                for k in 0..4 {

                    // from (i,j) to right
                    if j + k < COLS {
                        temp_r += self.match_ai_token(i, j + k);
                    }

                    // from (i,j) to top
                    if i + k < ROWS {
                        temp_t += self.match_ai_token(i + k, j);
                    }

                    // from (i,j) to top-right
                    if i + k < ROWS && j + k < COLS {
                        temp_tr += self.match_ai_token(i + k, j + k);
                    }

                    // from (i,j) to bottom-right
                    if i - k >= 0 && j + k < COLS {
                        temp_br += self.match_ai_token(i - k, j + k);
                    }
                }
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_t * temp_t * temp_t;
                chain_val += temp_tr * temp_tr * temp_tr;
                chain_val += temp_br * temp_br * temp_br;

                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } else if temp_t.abs() == 4 {
                    win_val = temp_t;
                } else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                } else if temp_br.abs() == 4 {
                    win_val = temp_br;
                }
            }
        }

        (win_val, chain_val)
    }

    fn value(&mut self, depth: isize, alpha: isize, beta: isize) -> (isize, isize) {
        let (win_val, chain_val) = self.check_state();

        // if slow (or memory consumption is high), lower the value
        if depth >= 4 {
            let mut ret_value = chain_val * AI_MOVE_VALUE;

            if win_val == 4 * AI_MOVE_VALUE {
                ret_value = 999999;
            } else if win_val == 4 * AI_MOVE_VALUE * -1 {
                ret_value = -999999;
            }

            ret_value -= depth * depth;
            return (ret_value, -1 as isize)
        }

        // AI won
        if win_val == 4 * AI_MOVE_VALUE {
            return (999999 - depth * depth, -1 as isize)
        }
        // AI lost
        if win_val == 4 * AI_MOVE_VALUE * -1 {
            return (-999999 - depth * depth, -1 as isize)
        }

        if depth % 2 == 0 {
            return self.min_state(depth + 1, alpha, beta);
        }

        self.max_state(depth + 1, alpha, beta)
    }

    fn choose(&self, choices: Vec<isize>) -> Dim {
        let temp = vec![2];
        let addr = &temp as *const Vec<i32>;
        let index = addr as usize;
        if choices.len() == 0 {
            return ((index % 6) as isize).abs()
        }
        choices[index % choices.len()].abs()
    }

    fn max_state(&mut self, depth: isize, mut alpha: isize, beta: isize) -> (isize, isize) {
        let mut v = -isize::max_value();
        let mut _move = -1;
        let mut move_queue = Vec::new();

        // for each column
        for j in 0..COLS {
            let inserted = self.insert_in_col(j, self.token);
            if inserted {
                let (temp_val, _) = self.value(depth, alpha, beta);
                if temp_val > v {
                    v = temp_val;

                    // restart queue cause this column is the best move?
                    move_queue.clear();
                    move_queue.push(j);
                } else if temp_val == v {
                    move_queue.push(j);
                }
                
                // alpha-beta pruning
                if v > beta {
                    _move = self.choose(move_queue);
                    return (v, _move); // (value, col)
                }
                alpha = max(alpha, v);
            }
        }

        _move = self.choose(move_queue);
        (v, _move)
    }

    fn min_state(&mut self, depth: isize, alpha: isize, mut beta: isize) -> (isize, isize) {
        let mut v = isize::max_value();
        let mut _move = -1;
        let mut move_queue = Vec::new();

        // for each column
        for j in 0..COLS {
            let inserted = self.insert_in_col(j, Token::O);
            if inserted {
                let (temp_val, _) = self.value(depth, alpha, beta);
                if temp_val < v {
                    v = temp_val;
                    _move = j;

                    // restart queue cause this column is the best move?
                    move_queue.clear();
                    move_queue.push(j);
                } else if temp_val == v {
                    move_queue.push(j);
                }

                // alpha-beta pruning
                if v < alpha {
                    _move = self.choose(move_queue);
                    return (v, _move);
                }
                beta = min(beta, v);
            }
        }
        _move = self.choose(move_queue);
        (v, _move)
    }
}