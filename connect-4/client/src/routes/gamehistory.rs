#![recursion_limit = "256"]

use yew::prelude::*;
use serde_json::json;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use anyhow::Error;

pub struct GameHistory {
    link: ComponentLink<Self>,
    ft: Option<FetchTask>,
    state: State,
}

pub struct State {
    link: ComponentLink<GameHistory>,
    fetching: bool, // for when the server says "one moment please"
    json_value: Option<String>, // TODO: replace String with Game struct
}

pub enum Msg {
    Fetch,
    FetchComplete(Result<String, Error>), // TODO: replace string with Game and refactor
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
        html! {
            <div>
                <h1><b>{"Game History"}</b></h1>
                <div>
                    <h4>{ self.view_history() }</h4>
                </div>
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
}

impl GameHistory {
    
    fn view_history(&self) -> Html {
        if let Some(value) = &self.state.json_value {
            html! {
                <>
                // TODO: format game history i guess
                { value }
                </>
            }
        } else {
            html! {
                <p>{ "spare fetch?" }</p>
            }
        }
    }
}

impl State {
    fn get_history(&mut self) -> FetchTask {
        let get_request = Request::get("/games")
                .body(Nothing).expect("Failed to build request");
        
        let callback = self.link.callback(
            | response: Response<Json<Result<String, Error>>> | {
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