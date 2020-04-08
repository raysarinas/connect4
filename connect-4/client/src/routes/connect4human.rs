use yew::prelude::*;

pub struct Connect4Human {
    link: ComponentLink<Self>,
    player1_name: String,
    player2_name: String
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
            player2_name: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotPlayer1Name(name) => self.player1_name = name,
            Msg::GotPlayer2Name(name) => self.player2_name = name,
            Msg::StartGame => {

            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
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
        }
    }
}