use crate::game_elements::*;

use yew::prelude::*;

pub struct TootOttoBoard {
    link: ComponentLink<Self>,
    props: Props,
    selected_token: Token,
    turn: Turn,
    clicked_tokens: Vec<Vec<bool>>,
}

#[derive(PartialEq)]
pub enum Token {
    T,
    O
}

pub enum Msg {
    GotToken(Token),
    Clicked(usize, usize)
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub player1_name: String,
    pub player2_name: String,
    pub difficulty: Difficulty
}

impl TootOttoBoard {
    fn draw_token(&self) -> String {
        match &self.selected_token {
            T => "T".to_string(),
            O => "O".to_string(),
        }
    }
}

impl Component for TootOttoBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TootOttoBoard {
            link,
            props: props,
            selected_token: Token::T,
            turn: Turn::First,
            clicked_tokens: vec![vec![false; 7]; 4],
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
                // draw token here
                if !self.clicked_tokens[row][col] {
                    self.clicked_tokens[row][col] = true;
                }
                self.turn.next();
            }
        }
        true
    }

    fn view(&self) -> Html {
        let col = |r, c| {
            html! {
                <td class="board_column" onclick=self.link.callback(move |_| Msg::Clicked(r, c))>
                <div hidden=!self.clicked_tokens[r][c] >{ self.draw_token() }</div>
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
                    {row(0)}
                    {row(1)}
                    {row(2)}
                    {row(3)}
                </table>
                <br></br>
            </div>
        }
    }
}

impl TootOttoBoard {
    fn find_winner() {

    }
}