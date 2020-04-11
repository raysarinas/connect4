use yew::prelude::*;
use serde_json::json;
use serde_json::ser;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;
use bson::UtcDateTime;
use chrono::{DateTime, Utc, NaiveDateTime};
use std::collections::HashMap;
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

        let parse_date = |date: String| {
            let ms: i64 = date.parse::<i64>().unwrap();
            let seconds = (ms / 1000) as i64;
            let nanos = ((ms % 1000) * 1_000_000) as u32;
            let naive_datetime = NaiveDateTime::from_timestamp(seconds, nanos);
            let stamp: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
            stamp.format("%H:%M on %b %e, %Y").to_string()
        };
        
        let comprow = |(index, g): (usize, &Game)| {
            html! {
                <tr>
                    <td>{index + 1}</td>
                    <td>{g.gameType.clone()}</td>
                    <td>{g.WinnerName.clone()}</td>
                    <td>{g.Player1Name.clone()}</td>
                    <td>{parse_date(g.GameDate.clone())}</td>
                </tr>
            }
        };

        let winrow = |(index, (winner, num))| {
            html! {
                <tr>
                    <td>{index + 1}</td>
                    <td>{winner}</td>
                    <td>{num}</td>
                </tr>
            }
        };

        // let sorted = |map: HashMap<String, usize>| {
        //     let mut v: Vec<_> = map.iter().collect();
        //     v.sort_by(|a, b| b.1.cmp(&a.1));
        //     v
        // };

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
                    </thead>
                        <tbody>
                        { for self.get_games_computer().iter().enumerate().map(comprow) }
                        </tbody>
                    </table>
                </div>
                <br></br>
                <div>
                    <h4>{"Details of Games Won by All Players"}</h4>
                    <table border="1">
                    <thead class = "thead-dark">
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                    </thead>
                        <tbody>
                        // sorted(map).to_vec()
                            { for self.get_win_map().iter().enumerate().map(winrow) }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}

impl ScoreBoard {

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

    // fn get_win_vec(&self) -> Vec<(String, usize)> {
    //     let map = self.get_win_map();
    //     // let mut v: Vec<_> = map.iter().collect();
    //     // v.sort_by(|a, b| b.1.cmp(&a.1));
    //     let sorted = |map: HashMap<String, usize>| {
    //         let mut v: Vec<_> = map.iter().collect();
    //         v.sort_by(|a, b| b.1.cmp(&a.1));
    //         v
    //     };
    //     sorted(map).to_vec()
    // }

    fn get_win_map(&self) -> HashMap<String, usize> {
        self.get_gamevec().iter()
            .fold(HashMap::new(), |mut acc, doc| {
                acc.entry(doc.WinnerName.clone())
                    .and_modify(|e| { *e += 1 })
                    .or_insert_with(|| 1);
            acc
        })
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