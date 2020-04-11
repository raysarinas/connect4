use std::collections::{HashSet, HashMap};
use crate::game_elements::*;
extern crate rand;

use rand::Rng;
use std::cmp::max;

pub struct Bot {
    token: Token,
    difficulty: Difficulty,
    board: GameBoard,
    // state: State, // maybe remove
}

// pub struct State { // maybe remove
//     winVal: isize,
//     chainVal: isize,
// }

pub type GameBoard = HashMap<Coord, Token>;

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    R,
    Y
}

impl Bot {
    const aiMoveValue: isize = -1;

    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            token: Token::Y,
            difficulty: difficulty,
            board: HashMap::new(),
            // state: State {
            //     winVal: 0,
            //     chainVal: 0,
            // }
        }
    }

    // pub fn set_board(board: &GameBoard) {
    //     self.board = board;
    // }

    fn matchAIToken(&self, board: &GameBoard, row: isize, col: isize) -> isize {
        let tempBoard = board.clone();
        if Self::token_is_at(&tempBoard, (row, col), &self.token) { // AI is negative
            -1 // AI Token
        } else if board.get(&(row, col)).is_none() {
            0 // Blank space
        } else {
            1 // Player's Token
        }
    }

    fn token_is_at(board: &GameBoard, coord: Coord, expected: &Token) -> bool {
        match board.get(&coord) {
            Some(actual) => expected == actual,
            None => false,
        }
    }

    fn fillMap(&self, state: &GameBoard, col: isize, value: isize) -> Option<GameBoard> {
        let mut tempMap = state.clone();
        if tempMap.get(&(0, col)).is_some() || col < 0 || col > 6 {
            return None
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if tempMap.get(&(i + 1, col)).is_some() {
                done = true;
                row = i;
                break;
            }
        }

        if !done {
            row = 5;
        }
        tempMap.insert((row, col), self.token);
        Some(tempMap)
    }

    fn checkState(&self, state: &GameBoard) -> (isize, isize) {
        let board = state.clone();
        let rows = 6; // 6, i
        let cols = 7; // 7, j
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
                        temp_r += self.matchAIToken(&board, i, j + k);
                    }

                    // from (i,j) to bottom
                    if i + k < rows {
                        temp_b += self.matchAIToken(&board, i + k, j);
                    }

                    // from (i,j) to bottom-right
                    if i + k < rows && j + k < cols {
                        temp_br += self.matchAIToken(&board, i + k, j + k);
                    }

                    // from (i,j) to top-right
                    if i - k >= 0 && j + k < cols {
                        temp_tr += self.matchAIToken(&board, i - k, j + k);
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
            return self.minState(&board, depth + 1, alpha, beta);
        }

        self.maxState(&board, depth + 1, alpha, beta)
    }

    fn choose(choices: Vec<isize>) -> isize {
        let mut rng = rand::thread_rng();
        choices[rng.gen_range(0, choices.len())]
    }

    fn maxState(&self, board: &GameBoard, depth: isize, mut alpha: isize, mut beta: isize) -> (isize, isize) {
        let mut v = -isize::max_value();
        let mut _move = -1;
        let mut tempVal: (isize, isize);
        let mut moveQueue = Vec::new();

        for j in 0..7 {
            let mut tempState = self.fillMap(&board, j, Bot::aiMoveValue);
            if tempState.is_some() {
                tempVal = self.value(&tempState.unwrap(), depth, alpha, beta);
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

    fn minState(&self, board: &GameBoard, depth: isize, mut alpha: isize, mut beta: isize) -> (isize, isize) {
        let mut v = isize::max_value();
        let mut _move = -1;
        let mut tempVal: (isize, isize);
        let mut moveQueue = Vec::new();

        for j in 0..7 {
            let mut tempState = self.fillMap(&board, j, Bot::aiMoveValue * -1);
            if tempState.is_some() {
                tempVal = self.value(&tempState.unwrap(), depth, alpha, beta);
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