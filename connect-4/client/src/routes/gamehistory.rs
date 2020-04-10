use yew::prelude::*;
use serde_json::json;
use serde_json::ser;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;
use bson::UtcDateTime;
use chrono::{DateTime, Utc};

use common::{Game};

pub struct GameHistory {
    link: ComponentLink<Self>,
    ft: Option<FetchTask>,
    state: State,
}

pub struct State {
    link: ComponentLink<GameHistory>,
    fetching: bool, // for when the server says "one moment please"
    json_value: Option<Vec<String>>, // TODO: replace String with Game struct
}

pub enum Msg {
    Fetch,
    FetchComplete(Result<Vec<String>, Error>), // TODO: replace string with Game and refactor
    FetchFailed,
}

impl Component for GameHistory {
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
        let row = |(index, g): (usize, &Game)| {
            html! {
                <tr>
                    <td>{index + 1}</td>
                    <td>{g.gameType.clone()}</td>
                    <td>{g.Player1Name.clone()}</td>
                    <td>{g.Player2Name.clone()}</td>
                    <td>{g.WinnerName.clone()}</td>
                    <td>{g.GameDate.clone()}</td>
                </tr>
            }
        };

        html! {
            <div>
                <h1><b>{"Game History"}</b></h1>
                <div>
                    <table border="1">
                    <thead class="thead-dark">
                        <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                    </thead>
                        <tbody>
                            { for self.get_gamevec().iter().enumerate().map(row) }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}

impl GameHistory {
    fn get_agame(val: &str) -> Game {
        let game: Game = serde_json::from_str(val).unwrap();
        game
    }

    fn get_gamevec(&self) -> Vec<Game> {
        if let Some(value) = &self.state.json_value {
            value.clone().iter_mut().map(|g| GameHistory::get_agame(g)).collect::<Vec<Game>>()
        } else {
            Vec::new() // TODO: handle error here
        }
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