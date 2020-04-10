use crate::game_elements::Difficulty;

use yew::prelude::*;

pub struct TootOttoBoard {
    link: ComponentLink<Self>,
    selected_token: Token,
    player1: Player,
    player2: Player,
    difficulty: Difficulty
}

#[derive(PartialEq)]
pub enum Token {
    T,
    O
}

pub struct Player {
    name: String,
    token: Token
}

pub enum Msg {
    GotToken(Token),
    Clicked(usize, usize)
}

#[derive(Properties, Clone)]
pub struct Props {
    pub player1_name: String,
    pub player2_name: String,
    pub difficulty: Difficulty
}

impl Component for TootOttoBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(p: Self::Properties, link: ComponentLink<Self>) -> Self {
        let player1 = Player {
            name: p.player1_name,
            token: Token::T
        };

        let player2 = Player {
            name: p.player2_name,
            token: Token::O
        };

        TootOttoBoard {
            link,
            selected_token: Token::T,
            player1: player1,
            player2: player2,
            difficulty: p.difficulty
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotToken(token) => {self.selected_token = token}, // TODO: assign selected_token to whoever's turn it is
            Msg::Clicked(row, col) => {}
        }
        true
    }

    fn view(&self) -> Html {
        let col = |r, c| {
            html! {
                <td class="board_column" onclick=self.link.callback(move |_| Msg::Clicked(r, c))>{""}</td>
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