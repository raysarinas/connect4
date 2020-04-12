use crate::game_elements::*;
use crate::connect_four::*;
use crate::connect_four::bot::Bot;

use std::collections::HashMap;

use yew::prelude::*;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;
use common::Game;
use yew::services::console::ConsoleService;

pub struct ConnectFourBoard {
    link: ComponentLink<Self>,
    props: Props,
    board: HashMap<Coord, Token>,
    current_token: Token,
    game_over: bool,
    winner: String,
    ft: Option<FetchTask>,
    state: State,
    console: ConsoleService,
    bot: Bot,
}

pub struct State {
    link: ComponentLink<ConnectFourBoard>,
    fetching: bool,
    json_value: Option<Game>,
    console: ConsoleService,
}

pub enum Msg {
    Clicked(Dim),
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
            console: ConsoleService::new(),
        };

        ConnectFourBoard {
            link,
            props: props,
            board: HashMap::new(),
            current_token: Token::R,
            game_over: false,
            winner: "".into(),
            ft: None, //Some(task),
            state,
            console: ConsoleService::new(),
            bot: Bot::new()
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
            Msg::Clicked(col) => {
                if !self.game_over {
                    match self.drop(col, self.current_token) {
                        Ok(()) => self.current_token.next(),
                        Err(e) => self.console.log(format!("Err {}", e).as_ref())
                    }
                    
                    // AI stuff
                    if self.props.player2_name == "Computer" && !self.game_over {
                        self.console.log("Computer turn");
                        let depth = match self.props.difficulty {
                            Difficulty::Easy => 3,
                            Difficulty::Medium => 4,
                            Difficulty::Hard => 50
                        };

                        // choose a move until we get a valid column
                        // column is valid only if it's not full
                        loop {
                            let col_bot = self.bot.get_move(self.board.clone(), depth);
                            self.console.log(format!("col_bot: {}", col_bot).as_ref());
                            match self.drop(col_bot, Token::Y) {
                                Ok(()) => {
                                    self.current_token.next();
                                    break;
                                },
                                Err(e) => self.console.log(format!("Err {}", e).as_ref())
                            }
                        }
                    }
                }
            },
            Msg::Fetch => {
                self.console.log("fetching");
                let task = self.state.post_game(&self.props, &self.winner);
                self.ft = Some(task);
            },
            Msg::FetchComplete(body) => {
                self.console.log("fetching done");
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

        let get_cell_color = |r, c| match self.get_token_at((r,c)) {
            Some(token) => {
                match &token {
                    Token::R => "red",
                    Token::Y => "bright_yellow"
                }
            },
            None => ""
        };

        let col = |r, c| {
            html! {
                <td class=format!("board_column {}", get_cell_color(r, c)) onclick=self.link.callback(move |_| Msg::Clicked(c))>
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

        let turn = || match self.current_token {
            Token::R => &self.props.player1_name,
            Token::Y => &self.props.player2_name
        };

        let get_result = || if self.winner == "Tie" {
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
    fn drop(&mut self, col: Dim, token: Token) -> Result<(), &'static str> {
        match self.next_free_row(col) {
            Some(row) => {
                self.put_token_at((row, col), token);
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
    fn put_token_at(&mut self, coord: Coord, token: Token) {
        self.board.insert(coord, token);
    }

    // Get the token at a given location on the board
    fn get_token_at(&self, coord: Coord) -> Option<&Token> {
        self.board.get(&coord)
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
        self.console.log("CHECKING");
        let mut winner_token = self.find_winner();

        if let Some(token) = winner_token {
            let winner = if token == Token::R {
                &self.props.player1_name
            } else {
                &self.props.player2_name
            };

            self.winner = winner.to_string();
            self.game_over = true;
            self.console.log("game over");
            self.post_game();
            return;
        }

        if self.is_full() {
            self.winner = "Tie".to_string();
            self.game_over = true;
            self.post_game();
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
        self.console.log("posting game");

        let new_game = Game {
            gameNumber: "1".to_string(),
            gameType: "Connect-4".to_string(),
            Player1Name: props.player1_name.clone(),
            Player2Name: props.player2_name.clone(),
            WinnerName: winner.clone(),
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
        self.console.log("returning fetch task");
        FetchService::new().fetch(post_request, callback).unwrap()
    }
}