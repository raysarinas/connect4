use crate::game_elements::*;
use crate::toot_otto::*;
use crate::toot_otto::bot::Bot;

use std::collections::{HashSet, HashMap};

use yew::prelude::*;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::console::ConsoleService;
use anyhow::Error;
use common::Game;

pub struct TootOttoBoard {
    link: ComponentLink<Self>,
    props: Props,
    board: HashMap<Coord, Token>,
    turn_map: HashMap<Coord, Turn>,
    selected_token: Token,
    turn: Turn,
    game_over: bool,
    winner: String,
    ft: Option<FetchTask>,
    state: State,
    console: ConsoleService,
    bot: Bot,
}

pub struct State {
    link: ComponentLink<TootOttoBoard>,
    fetching: bool,
    json_value: Option<String>,
    console: ConsoleService,
}

pub enum Msg {
    GotToken(Token),
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

impl Component for TootOttoBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut state = State {
            link: link.clone(),
            fetching: true,
            json_value: None,
            console: ConsoleService::new(),
        };

        TootOttoBoard {
            link,
            props: props,
            board: HashMap::new(),
            turn_map: HashMap::new(),
            selected_token: Token::T,
            turn: Turn::First,
            game_over: false,
            winner: "".to_string(),
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
            Msg::GotToken(token) => {
                self.console.log("got token");
                if !self.game_over {
                    self.console.log("selected token assigned");
                    self.selected_token = token
                    
                }
            },
            Msg::Clicked(col) => {
                if !self.game_over {
                    self.console.log("clicked");
                    match self.drop(col, self.selected_token, self.turn) {
                        Ok(()) => self.turn.next(),
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
                            let token = self.bot.get_token();
                            match self.drop(col_bot, token, self.turn) {
                                Ok(()) => {
                                    self.turn.next();
                                    break
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
                self.console.log("fetch completed");
                self.state.fetching = false;
                self.state.json_value = body.map(|data| data).ok();
            },
            Msg::FetchFailed => {
                self.console.log("fetch failed");
                self.state.json_value = None;
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let draw_token = |r, c| match self.get_token_at((r,c)) {
            Some(token) => {
                match &token {
                    Token::T => "T",
                    Token::O => "O"
                }
            },
            None => ""
        };

        let get_cell_color = |r, c| match self.get_turn_at((r,c)) {
            Some(turn) => {
                match &turn {
                    Turn::First => "green",
                    Turn::Second => "yellow"
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
                </tr>
            }
        };

        let turn = || match self.turn {
            Turn::First => &self.props.player1_name,
            Turn::Second => &self.props.player2_name
        };

        let get_result = || if self.winner == "Tie" {
            html! { <h1><b>{"It's a draw"}</b></h1> }
        } else {
            html! { <h1><b>{&self.winner}{" wins"}</b></h1> }
        };

        html! {
            <div>
                <div>
                    <form>
                        <h4>{"Select a Disc Type   : "}
                            <input
                                name="token"
                                type="radio"
                                id="T"
                                checked={self.selected_token == Token::T}
                                onclick=self.link.callback(|_| Msg::GotToken(Token::T))/>
                            <label for="T">{" T "}</label>
                            <input
                                name="token"
                                type="radio"
                                id="O"
                                checked={self.selected_token == Token::O}
                                onclick=self.link.callback(|_| Msg::GotToken(Token::O))/>
                            <label for="O">{" O"}</label>
                        </h4>
                    </form>
                    <h4>{"Turn   : "}{turn()}</h4>
                </div>
                <div hidden=!self.game_over>
                    {get_result()}
                </div>
                <table class="board">
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

impl TootOttoBoard {
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
        for row in 0..4 {
            if self.get_token_at((row, col)).is_none() {
                return Some(row);
            }
        }

        None
    }

    // check if the board is full by checking if there is a token 
    // at every col of the top row
    fn is_full(&self) -> bool {
        let top_row = 3;

        for col in 0..6 {
            if self.get_token_at((top_row, col)).is_none() {
                return false;
            }
        };
        
        true
    }

    fn check(&mut self) {
        self.console.log("checking winner");
        let winners = self.find_winners();

        match winners.len() {
            2 => {
                // self.console.log("winner len = 2");
                self.winner = "Tie".to_string();
                self.game_over = true;
                self.console.log("game over 2");
                return;
            }
            1 => {
                // self.console.log("winner len = 1");
                let winner = if winners.contains(&Token::T) {
                    &self.props.player1_name
                } else {
                    &self.props.player2_name
                };

                self.winner = winner.to_string();
                let log_winner = self.winner.clone();
                self.console.log(&log_winner);
                self.game_over = true;
                self.console.log("game over 1");
                // self.update(Msg::Fetch);
                self.post_game();
                return;
            }
            _ => {}
        };

        if self.is_full() {
            self.winner = "Tie".to_string();
            self.game_over = true;
            self.post_game();
        }
    }

    fn post_game(&mut self) {
        self.console.log("calling post game");
        self.state.post_game(&self.props, &self.winner);
        // self.update(Msg::Fetch);
    }

    fn find_winners(&self) -> HashSet<&Token> {
        let rows = 4;
        let cols = 6;
        let mut winners_set = HashSet::new();

        for row in 0..rows {
            for col in 0..cols {
                let token_opt = self.get_token_at((row, col));

                if token_opt.is_none() {
                    continue;
                }

                let token = token_opt.unwrap();
                let opposite = match &token {
                    Token::T => &Token::O,
                    Token::O => &Token::T
                };

                // Check right
                if col + 3 < cols
                    && self.token_is_at((row, col + 1), opposite)
                    && self.token_is_at((row, col + 2), opposite)
                    && self.token_is_at((row, col + 3), token)
                {
                    winners_set.insert(token);
                    continue;
                }

                if row + 3 < rows {
                    // Check up
                    if self.token_is_at((row + 1, col), opposite)
                        && self.token_is_at((row + 2, col), opposite)
                        && self.token_is_at((row + 3, col), token)
                    {
                        winners_set.insert(token);
                        continue;
                    }

                    // Check up and right
                    if col + 3 < cols
                        && self.token_is_at((row + 1, col + 1), opposite)
                        && self.token_is_at((row + 2, col + 2), opposite)
                        && self.token_is_at((row + 3, col + 3), token)
                    {
                        winners_set.insert(token);
                        continue;
                    }

                    // Check up and left
                    if col - 3 >= 0
                        && self.token_is_at((row + 1, col - 1), opposite)
                        && self.token_is_at((row + 2, col - 2), opposite)
                        && self.token_is_at((row + 3, col - 3), token)
                    {
                        winners_set.insert(token);
                        continue;
                    }
                }
            }
        }

        winners_set
    }
}

impl State {
    fn post_game(&mut self, props: &Props, winner: &String) -> FetchTask {
        self.console.log("in post game");

        let new_game = Game {
            gameNumber: "1".to_string(),
            gameType: "TOOT-OTTO".to_string(),
            Player1Name: props.player1_name.clone(),
            Player2Name: props.player2_name.clone(),
            WinnerName: winner.clone(),
            GameDate: "".to_string(),
        };

        self.console.log("new game made, calling post req");
        let post_request = Request::post("/games")
            .body(Json(&new_game))
            .expect("Failed to build request");

        self.console.log("now assigning callback");

        let handler = move |_response: Response<Json<Result<String, Error>>>| {
            panic!("Failed to send request");
        };

        FetchService::new().fetch(post_request, handler.into()).unwrap()
    }
}