#![recursion_limit="512"]

use crate::routes::{
    howtoconnect4::HowToConnect4,
    connect4comp::Connect4Computer,
    connect4human::Connect4Human,
    howtotoot::HowToTootOtto,
    tootottocomp::TootOttoComputer,
    tootottohuman::TootOttoHuman,
    gamehistory::GameHistory,
    scoreboard::ScoreBoard,
    home::Home
};

mod nav_bar;
mod routes;

use nav_bar::*;
use routes::*;

use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <NavigationBar />
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::HowToConnect4 => html!{<HowToConnect4 />},
                            AppRoute::Connect4Computer => html!{<Connect4Computer />},
                            AppRoute::Connect4Human => html!{<Connect4Human />},
                            AppRoute::HowToTootOtto => html!{<HowToTootOtto />},
                            AppRoute::TootOttoComputer => html!{<TootOttoComputer />},
                            AppRoute::TootOttoHuman => html!{<TootOttoHuman />},
                            AppRoute::GameHistory => html!{<GameHistory />},
                            AppRoute::ScoreBoard => html!{<ScoreBoard />},
                            AppRoute::Home => html!{<Home />},
                        }
                    })
                />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}