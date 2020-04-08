use crate::routes::AppRoute;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_router::prelude::*;

// MODEL
pub struct NavigationBar {
    link: ComponentLink<Self>,
    tab: Tab,
    difficulty: Difficulty,
    player1_name: String,
    player2_name: String
}

pub enum Tab {
    TootOttoComputer,
    TootOttoHuman,
    GameHistory,
    ScoreBoard,
    Nothing
}

// could probably put this in a different file
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

// CONTROLLER
// condense the following Clicked.... to ClickedTab(Tab) if can resolve recursion issue
pub enum Msg {
    ClickedTootComp,
    ClickedTootHuman,
    ClickedGameHistory,
    ClickedScoreBoard,
    GotPlayer1Name(String),
    GotPlayer2Name(String),
    GotDifficulty(Difficulty),
    StartConnect4,
    StartConnect4Comp,
    StartTootOtto,
    StartTootOttoComp
}

impl Component for NavigationBar {
    type Message = Msg;
    type Properties = ();
    
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavigationBar {
            link,
            tab: Tab::Nothing,
            difficulty: Difficulty::Easy,
            player1_name: "".into(),
            player2_name: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickedTootComp => self.tab = Tab::TootOttoComputer,
            Msg::ClickedTootHuman => self.tab = Tab::TootOttoHuman,
            Msg::ClickedGameHistory => self.tab = Tab::GameHistory,
            Msg::ClickedScoreBoard => self.tab = Tab::ScoreBoard,
            Msg::GotPlayer1Name(name) => self.player1_name = name,
            Msg::GotPlayer2Name(name) => self.player2_name = name,
            Msg::GotDifficulty(difficulty) => self.difficulty = difficulty,
            Msg::StartConnect4 => {
                // i think we call callback emit here idk
            },
            Msg::StartConnect4Comp => {
                // i think we call callback emit here idk
            },
            Msg::StartTootOtto => {
                // i think we call callback emit here idk
            },
            Msg::StartTootOttoComp => {
                // i think we call callback emit here idk
            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        let view_tab = || match self.tab {
            Tab::TootOttoComputer => self.view_toototto_computer(),
            Tab::TootOttoHuman => self.view_toototto_human(),
            Tab::GameHistory => self.view_game_history(),
            Tab::ScoreBoard => self.view_scoreboard(),
            Tab::Nothing => self.view_main()
        };
        html! {
            <div>
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding menu" style="z-index:3;height:100%;width:350px;font-weight:bold" id="mySidenav">
                    <div class="w3-container">
                        <h3 class="w3-padding-64"><b>{"Play"}<br></br>{"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>
                    <RouterAnchor<AppRoute> route=AppRoute::HowToConnect4 classes="nav-link w3-padding w3-hover-white">{"How to Play Connect 4"}</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Connect4Computer classes="nav-link w3-padding w3-hover-white">{"Play Connect4 with Computer"}</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Connect4Human classes="nav-link w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::HowToTootOtto classes="nav-link w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</RouterAnchor<AppRoute>>
                    <a href="#/TootOttoComputer" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedTootComp)>{"Play Toot-Otto with Computer"}</a><br></br>
                    <a href="#/TootOttoHuman" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedTootHuman)>{"Play Toot-Otto With Another Human"}</a><br></br>
                    <a href="#/GameHistory" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedGameHistory)>{"View Game History"}</a><br></br>
                    <a href="#/ScoreBoard" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedScoreBoard)>{"Score Board"}</a>
                </nav>
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                    {view_tab()}
                </div>
            </div>
        }
    }
}


impl NavigationBar {
    fn view_toototto_computer(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Enter Your Name"}</b></h1>
                <div>
                    <input
                        type="text"
                        value=&self.player1_name
                        oninput=self.link.callback(|e: InputData| Msg::GotPlayer1Name(e.value))
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
                <button onclick=self.link.callback(|_| Msg::StartTootOtto)>{"Start Game"}</button>
            </div>
        }
    }

    fn view_toototto_human(&self) -> Html {
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
                    <button onclick=self.link.callback(|_| Msg::StartTootOttoComp)>{"Start Game"}</button>
                </div>
            </div>
        }
    }

    fn view_game_history(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Game History"}</b></h1>
                <div>
                    <table border="1">
                        <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        <tr>
                            // populate here i guess
                        </tr>
                    </table>
                </div>
            </div>
        }
    }

    fn view_scoreboard(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Score Board"}</b></h1>
                <div>
                    <h4>{"Games Won by Computer"}</h4>
                    <table border="1">
                        <tr>
                            <th>{"Total Games Played"}</th>
                            <th>{"Games Against Computer"}</th>
                            <th>{"Games Computer Won"}</th>
                        </tr>
                        <tr>
                            // populate here i guess
                        </tr>
                    </table>
                </div>
                <br></br>
                <div>
                    <h4>{"Details of Games Won by Computer"}</h4>
                    <table border="1">
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Winner"}</th>
                            <th>{"Played Against"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        <tr>
                            // populate here i guess
                        </tr>
                    </table>
                </div>
                <br></br>
                <div>
                    <h4>{"Details of Games Won by All Players"}</h4>
                    <table border="1">
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        <tr>
                            // populate here i guess
                        </tr>
                    </table>
                </div>
            </div>
        }
    }

    fn view_main(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Welcome"}</b></h1>
                <p>{"This application contains the following two board games, both in human vs. human and human vs. computer versions."}</p>
                <ul>
                    <li>{"Connect 4"}</li>
                    <li>{"TOOT-OTTO"}</li>
                </ul>
                <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
            </div>
        }
    }
}

