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

pub struct Model {
    link: ComponentLink<Self>,
    route: Option<AppRoute>
}

pub enum Msg {
    Route(Route)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();

        Model {
            link,
            route: AppRoute::switch(route)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => self.route = AppRoute::switch(route)
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <NavigationBar />
                {
                    if let Some(route) = &self.route {
                        match route {
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
                    } else {
                        html!{"ERRRORRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR"}
                    }
                }
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