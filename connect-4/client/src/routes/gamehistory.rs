use yew::prelude::*;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;
use chrono::{DateTime, Utc, NaiveDateTime};
use std::collections::HashMap;
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
        let parse_date = |date: String| {
            let ms: i64 = date.parse::<i64>().unwrap();
            let seconds = (ms / 1000) as i64;
            let nanos = ((ms % 1000) * 1_000_000) as u32;
            let naive_datetime = NaiveDateTime::from_timestamp(seconds, nanos);
            let stamp: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
            stamp.format("%H:%M on %b %e, %Y").to_string()
        };

        let row = |(index, g): (usize, &Game)| {
            html! {
                <tr>
                    <td>{index + 1}</td>
                    <td>{g.gameType.clone()}</td>
                    <td>{g.Player1Name.clone()}</td>
                    <td>{g.Player2Name.clone()}</td>
                    <td>{g.WinnerName.clone()}</td>
                    <td>{parse_date(g.GameDate.clone())}</td>
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
        let split = val.replace("\"", "")
                .replace("{_id:{$", "").replace("{$date:{$numberLong:", "").replace("}", "")
                .split(',')
                .map(|kv| kv.split(':'))
                .map(|mut kv| (kv.next().unwrap().into(),
                            kv.next().unwrap().into()))
                .collect::<HashMap<String, String>>();

        Game {
            gameNumber: split["gameNumber"].clone(),
            gameType: split["gameType"].clone(),
            Player1Name: split["Player1Name"].clone(),
            Player2Name: split["Player2Name"].clone(),
            WinnerName: split["WinnerName"].clone(),
            GameDate: split["GameDate"].clone(),
        }
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