use crate::connect_four::Board;
use crate::game_board::GameBoard;
use crate::game_elements::*;
use crate::game_player::GamePlayer;
extern crate rand;

use rand::Rng;
use std::cmp::max;

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

pub struct State {
    winVal: isize,
    chainVal: isize,
}

pub struct Bot<'a> {
    name: String,
    token: Token,
    dept: usize,
    board: &'a dyn GameBoard,
    state: State,
}

impl<'b> Bot<'b> {
    const aiMoveValue: isize = -1;

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
            state: State {
                winVal: 0,
                chainVal: 0,
            }
        }
    }

    fn matchAIToken(&self, board: &GameBoard, row: isize, col: isize) -> isize {
        if board.token_is_at((row, col), &self.token) { // AI is negative
            -1
        } else {
            1
        }
    }


    fn fillMap(&self, state: &GameBoard, col: isize, value: isize) -> Option<&GameBoard> {
        let tempMap = state.clone();
        if tempMap.get_token_at((0, col)).is_some() || col < 0 || col > 6 {
            return None
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if tempMap.get_token_at((i + 1, col)).is_some() {
                done = true;
                row = i;
                break;
            }
        }

        if !done {
            row = 5;
        }
        //tempMap.insert((row, col), self.token);
        Some(tempMap)
    }

    fn checkState(&self, state: &GameBoard) -> (isize, isize) {
        let board = state.clone();
        let rows = board.size().rows(); // 6, i
        let cols = board.size().cols(); // 7, j
        let mut winVal = 0;
        let mut chainVal = 0;
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
                for k in 0..3 {

                    // from (i,j) to right
                    if j + k < cols {
                        temp_r += self.matchAIToken(board, i, j + k);
                    }

                    // from (i,j) to bottom
                    if i + k < rows {
                        temp_b += self.matchAIToken(board, i + k, j);
                    }

                    // from (i,j) to bottom-right
                    if i + k < rows && j + k < cols {
                        temp_br += self.matchAIToken(board, i + k, j + k);
                    }

                    // from (i,j) to top-right
                    if i - k >= 0 && j + k < cols {
                        temp_tr += self.matchAIToken(board, i - k, j + k);
                    }
                }
                chainVal += temp_r * temp_r * temp_r;
                chainVal += temp_b * temp_b * temp_b;
                chainVal += temp_br * temp_br * temp_br;
                chainVal += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    winVal = temp_r;
                } else if temp_b.abs() == 4 {
                    winVal = temp_b;
                } else if temp_br.abs() == 4 {
                    winVal = temp_br;
                } else if temp_tr.abs() == 4 {
                    winVal = temp_tr;
                }
            }
        }

        (winVal, chainVal)
    }

    fn value(&self, board: &GameBoard, depth: isize, alpha: isize, beta: isize) -> (isize, isize) {
        let val = self.checkState(board);
        if depth >= 4 {
            let mut retValue = 0;
            let winVal = val.0;
            let chainVal = val.1 * Bot::aiMoveValue;
            retValue = chainVal;

            if winVal == 4 * Bot::aiMoveValue {
                retValue = 999999;
            } else if winVal == 4 * Bot::aiMoveValue * -1 {
                retValue = -999999;
            }

            retValue -= depth * depth;
            return (retValue, -1 as isize)
        }

        let win = val.0;

        if win == 4 * Bot::aiMoveValue {
            return (999999 - depth * depth, -1 as isize)
        }
        if win == 4 * Bot::aiMoveValue * -1 {
            return (-999999 - depth * depth, -1 as isize)
        }

        if depth % 2 == 0 {
            return self.minState(depth + 1, alpha, beta);
        }

        self.maxState(depth + 1, alpha, beta)
    }

    fn choose(choices: Vec<isize>) -> isize {
        let mut rng = rand::thread_rng();
        choices[rng.gen_range(0, choices.len())]
    }

    fn maxState(&self, depth: isize, alpha: isize, beta: isize) -> (isize, isize) {
        let v = -100000000007;
        let _move = -1;
        let mut tempVal: (isize, isize);
        let mut moveQueue = Vec::new();

        for j in 0..self.board.size().rows() {
            let mut tempState = self.fillMap(self.board, j, Bot::aiMoveValue);
            if tempState.is_some() {
                tempVal = self.value(tempState.unwrap(), depth, alpha, beta);
                if tempVal.0 > v {
                    v = tempVal.0;
                    _move = j;
                    moveQueue.clear();
                    moveQueue.push(j);
                } else if tempVal.0 == v {
                    moveQueue.push(j);
                }


                // alpha-beta pruning
                if v > beta {
                    _move = Self::choose(moveQueue);
                    return (v, _move);
                }
                alpha = max(alpha, v);
            }
        }
        _move = Self::choose(moveQueue);
        (v, _move)
    }

    fn minState(&self, depth: isize, alpha: isize, beta: isize) -> (isize, isize) {
        let v = 100000000007;
        let _move = -1;
        let mut tempVal: (isize, isize);
        let mut moveQueue = Vec::new();

        for j in 0..self.board.size().rows() {
            let mut tempState = self.fillMap(self.board, j, Bot::aiMoveValue * -1);
            if tempState.is_some() {
                tempVal = self.value(tempState.unwrap(), depth, alpha, beta);
                if tempVal.0 < v {
                    v = tempVal.0;
                    _move = j;
                    moveQueue.clear();
                    moveQueue.push(j);
                } else if tempVal.0 == v {
                    moveQueue.push(j);
                }


                // alpha-beta pruning
                if v < alpha {
                    _move = Self::choose(moveQueue);
                    return (v, _move);
                }
                beta = max(beta, v);
            }
        }
        _move = Self::choose(moveQueue);
        (v, _move)
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
