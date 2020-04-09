use yew::prelude::*;
use serde_json::json;
use serde_json::ser;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;
use bson::UtcDateTime;
use chrono::{DateTime, Utc};

use common::{Game};

pub struct ScoreBoard {
    link: ComponentLink<Self>,
    ft: Option<FetchTask>,
    state: State,
}

pub struct State {
    link: ComponentLink<ScoreBoard>,
    fetching: bool, // for when the server says "one moment please"
    json_value: Option<Vec<String>>, // TODO: replace String with Game struct
}

pub enum Msg {
    Fetch,
    FetchComplete(Result<Vec<String>, Error>), // TODO: replace string with Game and refactor
    FetchFailed,
}

impl Component for ScoreBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut state = State {
            link: link.clone(),
            fetching: true,
            json_value: None,
        };

        // fetch JSON at the start
        let task = state.get_history();
        Self {
            link,
            ft: Some(task),
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Fetch => {
                let task = self.state.get_history();
                self.ft = Some(task);
            },
            Msg::FetchComplete(body) => {
                self.state.fetching = false;
                self.state.json_value = body.map(|data| data).ok();
            },
            Msg::FetchFailed => {
                self.state.json_value = None;
                return false;
            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Score Board"}</b></h1>
                <div>
                    <h4>{"Games Won by Computer"}</h4>
                    <table border="1">
                    <thead class="thead-dark">
                        <tr>
                            <th>{"Total Games Played"}</th>
                            <th>{"Games Against Computer"}</th>
                            <th>{"Games Computer Won"}</th>
                        </tr>
                    </thead>
                        <tbody>
                        <tr>
                            <td>{ self.get_num_games_total() }</td>
                            <td>{ self.get_games_computer().len().to_string() }</td>
                            <td>{ self.get_games_won("Computer").len().to_string() }</td>
                        </tr>
                        </tbody>
                    </table>
                </div>
                <br></br>
                <div>
                    <h4>{"Details of Games Won by Computer"}</h4>
                    <table border="1">
                    <thead class="thead-dark">
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
                    </thead>
                        <tbody>
                        <tr>
                            <td>{ "HUH" }</td>
                            <td>{ "Type" }</td>
                            <td>{ "Winner" }</td>
                            <td>{ "Against" }</td>
                            <td>{ "When" }</td>
                        </tr>
                        </tbody>
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
}

impl ScoreBoard {

    fn get_agame(val: &str) -> Game {
        let Game: Game = serde_json::from_str(val).unwrap();
        Game
    }

    fn get_gamevec(&self) -> Vec<Game> {
        if let Some(value) = &self.state.json_value {
            value.clone().iter_mut().map(|g| ScoreBoard::get_agame(g)).collect::<Vec<Game>>()
        } else {
            Vec::new() // TODO: handle error here
        }
    }

    fn get_games_won(&self, winner: &str) -> Vec<Game> {
        self.get_gamevec().into_iter()
        .filter(|g| g.WinnerName == winner.to_string())
        .collect()
    }

    fn get_games_computer(&self) -> Vec<Game> {
        self.get_gamevec().into_iter()
            .filter(|g| g.Player2Name == "Computer".to_string())
            .collect()
    }

    fn get_num_games_total(&self) -> String {
        self.get_gamevec().len().to_string()
    }

}

impl State {
    fn get_history(&mut self) -> FetchTask {
        let get_request = Request::get("/games")
                .body(Nothing).expect("Failed to build request");
        
        let callback = self.link.callback(
            | response: Response<Json<Result<Vec<String>, Error>>> | {
                let (meta, Json(body)) = response.into_parts();
                if meta.status.is_success() {
                    return Msg::FetchComplete(body);
                }
                Msg::FetchFailed
            },
        );
        FetchService::new().fetch(get_request, callback).unwrap()
    }
}