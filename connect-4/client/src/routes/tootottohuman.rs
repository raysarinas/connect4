use crate::toot_otto::board::*;

use yew::prelude::*;

pub struct TootOttoHuman {
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

impl Component for TootOttoHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TootOttoHuman {
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
                    <div>
                        <h4>{"New Game: "}{&self.player1_name}{" vs. "}{&self.player2_name}</h4>
                        <small>{"(Winning Combination: "}{&self.player1_name}{" - "}<b>{"TOOT"}</b>{" and "}{&self.player2_name}{" - "}<b>{"OTTO"}</b>{")"}</small>
                    </div>
                    <br></br>
                    <TootOttoBoard />
                </div>
            </div>
        }
    }
}