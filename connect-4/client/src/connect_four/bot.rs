use crate::game_elements::*;
use crate::connect_four::*;

use std::collections::HashMap;
use std::cmp::max;

use yew::services::console::ConsoleService;

pub type GameBoard = HashMap<Coord, Token>;

pub struct Bot {
    token: Token,
    depth: isize,
    board: GameBoard,
    console: ConsoleService
}

impl Bot {
    const AI_MOVE_VALUE: isize = -1;

    pub fn new() -> Self {
        Bot {
            token: Token::Y,
            depth: 0,
            board: HashMap::new(),
            console: ConsoleService::new()
        }
    }

    pub fn get_move(&mut self, board: GameBoard, depth: isize) -> Dim {
        self.set_board(board);
        self.set_depth(depth);
        self.console.log(format!("depth: {}", &self.depth).as_ref());
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

    fn fill_map(&self, col: isize, value: isize) -> Option<GameBoard> {
        let mut temp_map = self.board.clone();
        if temp_map.get(&(0, col)).is_some() || col < 0 || col > 6 {
            return None
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if temp_map.get(&(i + 1, col)).is_some() {
                done = true;
                row = i;
                break;
            }
        }

        if !done {
            row = 5;
        }
        temp_map.insert((row, col), self.token);
        Some(temp_map)
    }

    fn check_state(&self) -> (isize, isize) {
        let rows = 6; // 6, i
        let cols = 7; // 7, j
        let mut win_val = 0;
        let mut chain_val = 0;
        let mut temp_r = 0;
        let mut temp_b = 0;
        let mut temp_br = 0;
        let mut temp_tr = 0;

        for i in 0..rows {
            for j in 0..cols {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..4 {

                    // from (i,j) to right
                    if j + k < cols {
                        temp_r += self.match_ai_token(i, j + k);
                    }

                    // from (i,j) to bottom
                    if i + k < rows {
                        temp_b += self.match_ai_token(i + k, j);
                    }

                    // from (i,j) to bottom-right
                    if i + k < rows && j + k < cols {
                        temp_br += self.match_ai_token(i + k, j + k);
                    }

                    // from (i,j) to top-right
                    if i - k >= 0 && j + k < cols {
                        temp_tr += self.match_ai_token(i - k, j + k);
                    }
                }
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } else if temp_b.abs() == 4 {
                    win_val = temp_b;
                } else if temp_br.abs() == 4 {
                    win_val = temp_br;
                } else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                }
            }
        }

        (win_val, chain_val)
    }

    fn value(&self, depth: isize, alpha: isize, beta: isize) -> (isize, isize) {
        let val = self.check_state();
        if depth >= 4 {
            let win_val = val.0;
            let mut ret_value = val.1 * Bot::AI_MOVE_VALUE;

            if win_val == 4 * Bot::AI_MOVE_VALUE {
                ret_value = 999999;
            } else if win_val == 4 * Bot::AI_MOVE_VALUE * -1 {
                ret_value = -999999;
            }

            ret_value -= depth * depth;
            return (ret_value, -1 as isize)
        }

        let win = val.0;

        if win == 4 * Bot::AI_MOVE_VALUE {
            return (999999 - depth * depth, -1 as isize)
        }
        if win == 4 * Bot::AI_MOVE_VALUE * -1 {
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
            return ((index % 7) as isize).abs()
        }
        choices[index % choices.len()].abs()
    }

    fn max_state(&self, depth: isize, mut alpha: isize, beta: isize) -> (isize, isize) {
        let mut v = -isize::max_value();
        let mut _move = -1;
        let mut temp_val: (isize, isize);
        let mut move_queue = Vec::new();

        for j in 0..7 {
            let temp_state = self.fill_map(j, Bot::AI_MOVE_VALUE);
            if temp_state.is_some() {
                temp_val = self.value(depth, alpha, beta);
                if temp_val.0 > v {
                    v = temp_val.0;
                    _move = j;
                    move_queue.clear();
                    move_queue.push(j);
                } else if temp_val.0 == v {
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

    fn min_state(&self, depth: isize, alpha: isize, mut beta: isize) -> (isize, isize) {
        let mut v = isize::max_value();
        let mut _move = -1;
        let mut temp_val: (isize, isize);
        let mut move_queue = Vec::new();

        for j in 0..7 {
            let temp_state = self.fill_map(j, Bot::AI_MOVE_VALUE * -1);
            if temp_state.is_some() {
                temp_val = self.value(depth, alpha, beta);
                if temp_val.0 < v {
                    v = temp_val.0;
                    _move = j;
                    move_queue.clear();
                    move_queue.push(j);
                } else if temp_val.0 == v {
                    move_queue.push(j);
                }


                // alpha-beta pruning
                if v < alpha {
                    _move = self.choose(move_queue);
                    return (v, _move);
                }
                beta = max(beta, v);
            }
        }
        _move = self.choose(move_queue);
        (v, _move)
    }
}