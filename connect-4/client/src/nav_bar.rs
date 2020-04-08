use crate::routes::AppRoute;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew_router::prelude::*;

// MODEL
pub struct NavigationBar {}

impl Component for NavigationBar {
    type Message = ();
    type Properties = ();
    
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavigationBar {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding menu" style="z-index:3;height:100%;width:350px;font-weight:bold" id="mySidenav">
                    <div class="w3-container">
                        <h3 class="w3-padding-64"><b>{"Play"}<br></br>{"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>
                    <RouterAnchor<AppRoute> route=AppRoute::HowToConnect4 classes="nav-link w3-padding w3-hover-white">{"How to Play Connect 4"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::Connect4Computer classes="nav-link w3-padding w3-hover-white">{"Play Connect4 with Computer"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::Connect4Human classes="nav-link w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::HowToTootOtto classes="nav-link w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::TootOttoComputer classes="nav-link w3-padding w3-hover-white">{"Play Toot-Otto with Computer"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::TootOttoHuman classes="nav-link w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::GameHistory classes="nav-link w3-padding w3-hover-white">{"View Game History"}</RouterAnchor<AppRoute>><br></br>
                    <RouterAnchor<AppRoute> route=AppRoute::ScoreBoard classes="nav-link w3-padding w3-hover-white">{"Score Board"}</RouterAnchor<AppRoute>>
                </nav>
            </div>
        }
    }
}