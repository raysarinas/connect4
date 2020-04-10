use crate::game_elements::*;

use std::collections::{HashSet, HashMap};

use yew::prelude::*;

pub struct TootOttoBoard {
    link: ComponentLink<Self>,
    props: Props,
    board: HashMap<Coord, Token>,
    selected_token: Token,
    turn: Turn
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Copy)]
pub enum Token {
    T,
    O
}

pub enum Msg {
    GotToken(Token),
    Clicked(isize, isize)
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
        TootOttoBoard {
            link,
            props: props,
            board: HashMap::new(),
            selected_token: Token::T,
            turn: Turn::First
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
            Msg::GotToken(token) => self.selected_token = token,
            Msg::Clicked(row, col) => {
                match self.drop(col, self.selected_token) {
                    Ok(()) => self.turn.next(),
                    Err(e) => println!("Err: {}", e), // TODO: do something with this
                }
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

        // TODO: color is wrong, fix
        let get_cell_color = |r, c| match self.get_token_at((r,c)) {
            Some(token) => {
                match &token {
                    Token::T => "green",
                    Token::O => "yellow"
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
                </tr>
            }
        };

        let turn = || match self.turn {
            Turn::First => &self.props.player1_name,
            Turn::Second => &self.props.player2_name
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
        for row in 0..4 {
            if self.get_token_at((row, col)).is_none() {
                return Some(row);
            }
        }

        None
    }

    // TODO: actually keep track of game_result and game_over
    fn check(&mut self) {
        let winners = self.find_winners();

        match winners.len() {
            2 => {
                // self.game_result = Some(GameResult::Tie);
                // self.game_over = true;
                return;
            }
            1 => {
                let winner = if winners.contains(&Token::T) {
                    Token::T
                } else {
                    Token::O
                };

                // self.game_result = Some(GameResult::Winner(winner));
                // self.game_over = true;
                return;
            }
            _ => {}
        };

        // if self.is_full() {
            // self.game_result = Some(GameResult::Tie);
            // self.game_over = true;
        // }
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