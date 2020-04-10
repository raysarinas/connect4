use yew::prelude::*;

pub struct TootOttoBoard {
    link: ComponentLink<Self>,
    token: Token
}

pub enum Msg {
    GotToken(Token),
}

#[derive(PartialEq)]
pub enum Token {
    T,
    O
}

impl Component for TootOttoBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TootOttoBoard {
            link,
            token: Token::T
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotToken(token) => self.token = token
        }
        true
    }

    fn view(&self) -> Html {
        let col = || {
            html! {
                <td class="board_column">{""}</td>
            }
        };

        let row = || {
            html! {
                <tr>
                    {col()}
                    {col()}
                    {col()}
                    {col()}
                    {col()}
                    {col()}
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
                                checked={self.token == Token::T}
                                onclick=self.link.callback(|_| Msg::GotToken(Token::T))/>
                            <label for="T">{" T "}</label>
                            <input
                                name="token"
                                type="radio"
                                id="O"
                                checked={self.token == Token::O}
                                onclick=self.link.callback(|_| Msg::GotToken(Token::O))/>
                            <label for="O">{" O"}</label>
                        </h4>
                    </form>
                </div>
                <table class="board">
                    {row()}
                    {row()}
                    {row()}
                    {row()}
                </table>
                <br></br>
            </div>
        }
    }
}