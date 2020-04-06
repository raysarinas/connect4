#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod connect_four;
mod game_board;
mod game_elements;
mod game_player;
mod toot_otto;

use game_elements::*;
use game_board::GameBoard;
use game_player::GamePlayer;

fn main() {
    // replace connect_four here with toot_otto or vice versa
    let mut board = connect_four::Board::new(5, 5);
    let mut player1 = connect_four::Player::new("player1", Token::R);
    let mut player2 = connect_four::Player::new("player2", Token::Y);
    // let mut connect_four_bot = connect_four::Bot::new(Token::R, Difficulty::Easy, &board);
    // let (col, token) = connect_four_bot.choose_move();

    // let mut board = toot_otto::Board::new(2, 4);
    // let mut player1 = toot_otto::Player::new("player1", Token::T);
    // let mut player2 = toot_otto::Player::new("player2", Token::O);
    // let mut toot_otto_bot = toot_otto::Bot::new(Token::T, Difficulty::Easy, &board);
    // let (col, token) = toot_otto_bot.choose_move();

    let mut turn = Turn::First;
    board.print();

    while !board.game_over() {
        let (col, token) = match turn {
            Turn::First => player1.choose_move(),
            Turn::Second => player2.choose_move(),
        };

        match board.drop(col, token) {
            Ok(()) => turn.next(),
            Err(e) => println!("Err: {}", e),
        }

        board.print();
    }

    match board.game_result() {
        GameResult::Winner(token) => {
            if token == player1.token() {
                println!("{} Wins!", player1.name());
            } else {
                println!("{} Wins!", player2.name());
            }
        }
        GameResult::Tie => println!("Tie"),
    };
}
