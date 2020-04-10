use crate::game_elements::*;

use std::collections::{HashSet, HashMap};

use yew::prelude::*;
use serde_json::json;
use serde_json::ser;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;
use bson::UtcDateTime;
use chrono::{DateTime, Utc};
use common::Game;

pub struct ConnectFourBoard {
    link: ComponentLink<Self>,
    props: Props,
    board: HashMap<Coord, Token>,
    turn_map: HashMap<Coord, Turn>,
    current_token: Token,
    turn: Turn,
    game_over: bool,
    winner: String,
    ft: Option<FetchTask>,
    state: State,
}

pub struct State {
    link: ComponentLink<ConnectFourBoard>,
    fetching: bool,
    json_value: Option<Game>,
}

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    R,
    Y
}

pub enum Msg {
    Clicked(Dim, Dim),
    Fetch,
    FetchComplete(Result<String, Error>),
    FetchFailed,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub player1_name: String,
    pub player2_name: String,
    pub difficulty: Difficulty
}

impl Component for ConnectFourBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut state = State {
            link: link.clone(),
            fetching: true,
            json_value: None,
        };

        ConnectFourBoard {
            link,
            props: props,
            board: HashMap::new(),
            turn_map: HashMap::new(),
            current_token: Token::Y,
            turn: Turn::First,
            game_over: false,
            winner: "".into(),
            ft: None, //Some(task),
            state,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props != props;
        if changed {
            self.props = props;
        }
        changed
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(row, col) => {
                if !self.game_over {
                    self.current_token = match self.current_token {
                        Token::R => Token::Y,
                        Token::Y => Token::R,
                    };

                    match self.drop(col, self.current_token, self.turn) {
                        Ok(()) => self.turn.next(),
                        Err(e) => println!("Err: {}", e), // TODO: do something with this
                    }

                    // if self.props.player2_name == "Computer", make ai move here i guess
                }
            },
            Msg::Fetch => {
                let task = self.state.post_game(&self.props, &self.winner);
                self.ft = Some(task);
            },
            Msg::FetchComplete(body) => {
                self.state.fetching = false;
                // self.state.json_value = body.map(|data| data).ok();
            },
            Msg::FetchFailed => {
                self.state.json_value = None;
                return false;
            }
        };
        true
    }

    fn view(&self) -> Html {
        let draw_token = |r, c| match self.get_token_at((r,c)) {
            Some(token) => {
                match &token {
                    Token::R => "R",
                    Token::Y => "Y"
                }
            },
            None => ""
        };

        let get_cell_color = |r, c| match self.get_turn_at((r,c)) {
            Some(turn) => {
                match &turn {
                    Turn::First => "red",
                    Turn::Second => "bright_yellow"
                }
            },
            None => ""
        };

        let col = |r, c| {
            html! {
                <td class=format!("board_column {}", get_cell_color(r, c)) onclick=self.link.callback(move |_| Msg::Clicked(r, c))>
                    {draw_token(r, c)}
                </td>
            }
        };

        let row = |r| {
            html! {
                <tr>
                    {col(r, 0)}
                    {col(r, 1)}
                    {col(r, 2)}
                    {col(r, 3)}
                    {col(r, 4)}
                    {col(r, 5)}
                    {col(r, 6)}
                </tr>
            }
        };

        let turn = || match self.turn {
            Turn::First => &self.props.player1_name,
            Turn::Second => &self.props.player2_name
        };

        let get_result = || if self.winner.is_empty() {
            html! { <h1><b>{"It's a draw"}</b></h1> }
        } else {
            html! { <h1><b>{&self.winner}{" wins"}</b></h1> }
        };

        html! {
            <div>
                <h4>{"Turn   : "}{turn()}</h4>
                <div hidden=!self.game_over>
                    {get_result()}
                </div>
                <table class="board">
                    {row(5)}
                    {row(4)}
                    {row(3)}
                    {row(2)}
                    {row(1)}
                    {row(0)}
                </table>
                <br></br>
            </div>
        }
    }
}


impl ConnectFourBoard {
    // Bottom left is 0, 0
    fn drop(&mut self, col: Dim, token: Token, turn: Turn) -> Result<(), &'static str> {
        match self.next_free_row(col) {
            Some(row) => {
                self.put_token_at((row, col), token, turn);
                self.check();
                Ok(())
            }
            None => Err("Column is full"),
        }
    }

    // Check if a token is at a given location on the board
    fn token_is_at(&self, coord: Coord, expected: &Token) -> bool {
        match self.get_token_at(coord) {
            Some(actual) => expected == actual,
            None => false,
        }
    }

    // set the token at a given location on the board
    // assert that there were no tokens there before
    fn put_token_at(&mut self, coord: Coord, token: Token, turn: Turn) {
        self.board.insert(coord, token);
        self.turn_map.insert(coord, turn);
    }

    // Get the token at a given location on the board
    fn get_token_at(&self, coord: Coord) -> Option<&Token> {
        self.board.get(&coord)
    }

    // Get the turn at a given location on the board
    fn get_turn_at(&self, coord: Coord) -> Option<&Turn> {
        self.turn_map.get(&coord)
    }

    // Given a col, check the next free row
    fn next_free_row(&self, col: Dim) -> Option<Dim> {
        for row in 0..6 {
            if self.get_token_at((row, col)).is_none() {
                return Some(row);
            }
        }

        None
    }

    // check if the board is full by checking if there is a token 
    // at every col of the top row
    fn is_full(&self) -> bool {
        let top_row = 5;

        for col in 0..7 {
            if self.get_token_at((top_row, col)).is_none() {
                return false;
            }
        };
        
        true
    }

    fn check(&mut self) {
        let mut winner_token = self.find_winner();

        if let Some(token) = winner_token {
            let winner = if token == Token::R {
                &self.props.player1_name
            } else {
                &self.props.player2_name
            };

            self.winner = winner.to_string();
            self.game_over = true;
            self.post_game();
        }

        if self.is_full() {
            self.game_over = true;
            // TODO: handle tie here?
        }
    }

    fn post_game(&mut self) {
        self.state.post_game(&self.props, &self.winner);
    }

    pub fn find_winner(&self) -> Option<Token> {
        let rows = 6;
        let cols = 7;

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


impl State {
    fn post_game(&mut self, props: &Props, winner: &String) -> FetchTask {
        let mut winner_str = || if winner.is_empty() {
            "Tie".to_string()
        } else {
            winner.clone()
        };

        let new_game = Game {
            gameNumber: "1".to_string(),
            gameType: "Connect-4".to_string(),
            Player1Name: props.player1_name.clone(),
            Player2Name: props.player2_name.clone(),
            WinnerName: winner_str(),
            GameDate: "".to_string(),
        };

        let post_request = Request::post("/games")
            .body(Json(&new_game))
            .expect("Failed to build request");

        let callback = self.link.callback(
            | response: Response<Result<String, Error>>| {
                if response.status().is_success() {
                    Msg::Fetch
                }
                else {
                    Msg::FetchFailed
                }
            },
        );
        FetchService::new().fetch(post_request, callback).unwrap()
    }
}