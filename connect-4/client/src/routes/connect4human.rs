use crate::connect_four::board::*;
use crate::game_elements::Difficulty;

use yew::prelude::*;

pub struct Connect4Human {
    link: ComponentLink<Self>,
    player1_name: String,
    player2_name: String,
    info_submitted: bool
}

pub enum Msg {
    GotPlayer1Name(String),
    GotPlayer2Name(String),
    StartGame
}

impl Component for Connect4Human {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Connect4Human {
            link,
            player1_name: "".into(),
            player2_name: "".into(),
            info_submitted: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotPlayer1Name(name) => self.player1_name = name,
            Msg::GotPlayer2Name(name) => self.player2_name = name,
            Msg::StartGame => {
                self.info_submitted = true;
                // disable start game button here
            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <h1><b>{"Enter Player Names"}</b></h1>
                    <div>
                        <input
                            type="text"
                            value=&self.player1_name
                            oninput=self.link.callback(|e: InputData| Msg::GotPlayer1Name(e.value))
                            placeholder="Player 1's Name"/>
                        <input
                            type="text"
                            value=&self.player2_name
                            oninput=self.link.callback(|e: InputData| Msg::GotPlayer2Name(e.value))
                            placeholder="Player 2's Name"/>
                        <button onclick=self.link.callback(|_| Msg::StartGame)>{"Start Game"}</button>
                    </div>
                </div>
                <br></br>
                <div hidden=!self.info_submitted>
                    <h4>{"New Game: "}{&self.player1_name}{" vs. "}{&self.player2_name}</h4>
                    <small>{"(Disc Colors: "}{&self.player1_name}{" - "}<b>{"Red"}</b>{" and "}{&self.player2_name}{" - "}<b>{"Yellow"}</b>{")"}</small>
                    <br></br>
                    <small>{"(Disc Type: "}{&self.player1_name}{" - "}<b>{"R"}</b>{" and "}{&self.player2_name}{" - "}<b>{"Y"}</b>{")"}</small>
                    <br></br>
                    <ConnectFourBoard player1_name=&self.player1_name player2_name=&self.player2_name, difficulty=&Difficulty::Easy />
                </div>
            </div>
        }
    }
}