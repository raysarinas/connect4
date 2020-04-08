use yew::prelude::*;

pub struct TootOttoComputer {
    link: ComponentLink<Self>,
    difficulty: Difficulty,
    player_name: String
}

pub enum Msg {
    GotPlayerName(String),
    GotDifficulty(Difficulty),
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
            player_name: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotPlayerName(name) => self.player_name = name,
            Msg::GotDifficulty(difficulty) => self.difficulty = difficulty,
            Msg::StartGame => {

            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
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
                    <select>
                        <option onclick=self.link.callback(|_| Msg::GotDifficulty(Difficulty::Easy))>{"Easy"}</option>
                        <option onclick=self.link.callback(|_| Msg::GotDifficulty(Difficulty::Medium))>{"Medium"}</option>
                        <option onclick=self.link.callback(|_| Msg::GotDifficulty(Difficulty::Hard))>{"Hard"}</option>
                    </select>
                </div>
                <br></br>
                <button onclick=self.link.callback(|_| Msg::StartGame)>{"Start Game"}</button>
            </div>
        }
    }
}