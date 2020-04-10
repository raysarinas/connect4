use crate::toot_otto::board::*;

use yew::prelude::*;

pub struct TootOttoComputer {
    link: ComponentLink<Self>,
    difficulty: Difficulty,
    player_name: String,
    info_submitted: bool
}

pub enum Msg {
    GotPlayerName(String),
    GotDifficulty(String),
    StartGame
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Component for TootOttoComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TootOttoComputer {
            link,
            difficulty: Difficulty::Easy,
            player_name: "".into(),
            info_submitted: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotPlayerName(name) => self.player_name = name,
            Msg::GotDifficulty(difficulty) => {
                match difficulty.as_ref() {
                    "Easy" => self.difficulty = Difficulty::Easy,
                    "Medium" => self.difficulty = Difficulty::Medium,
                    "Hard" => self.difficulty = Difficulty::Hard,
                    _ => unreachable!()
                }
            }
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
                    <h1><b>{"Enter Your Name"}</b></h1>
                    <div>
                        <input
                            type="text"
                            value=&self.player_name
                            oninput=self.link.callback(|e: InputData| Msg::GotPlayerName(e.value))
                            placeholder="Your Name"/>
                    </div>
                    <h1><b>{"Enter Difficulty"}</b></h1>
                    <div>
                        <select onchange=self.link.callback(|e: ChangeData| {
                            match e {
                                ChangeData::Select(s) => Msg::GotDifficulty(s.value().unwrap()),
                                _ => unreachable!()
                            }
                        })>
                            <option value="Easy">{"Easy"}</option>
                            <option value="Medium">{"Medium"}</option>
                            <option value="Hard">{"Hard"}</option>
                        </select>
                    </div>
                    <br></br>
                    <button onclick=self.link.callback(|_| Msg::StartGame)>{"Start Game"}</button>
                </div>
                <br></br>
                <div hidden=!self.info_submitted>
                    <div>
                        <h4>{"New Game: "}{&self.player_name}{" vs. Computer"}</h4>
                        <small>{"(Winning Combination: "}{&self.player_name}{" - "}<b>{"TOOT"}</b>{" and Computer - "}<b>{"OTTO"}</b>{")"}</small>
                    </div>
                    <br></br>
                    <TootOttoBoard />
                </div>
            </div>
        }
    }
}