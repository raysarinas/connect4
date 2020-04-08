//! Routes by yew_router

pub mod howtoconnect4;
pub mod connect4comp;
pub mod connect4human;
pub mod howtotoot;
pub mod tootottocomp;
pub mod tootottohuman;
pub mod gamehistory;
pub mod scoreboard;
pub mod home;

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/HowToConnect4"]
    HowToConnect4,
    #[to = "#/Connect4Computer"]
    Connect4Computer,
    #[to = "#/Connect4Human"]
    Connect4Human,
    #[to = "#/HowToToot"]
    HowToTootOtto,
    #[to = "#/TootOttoComputer"]
    TootOttoComputer,
    #[to = "#/TootOttoHuman"]
    TootOttoHuman,
    #[to = "#/GameHistory"]
    GameHistory,
    #[to = "#/ScoreBoard"]
    ScoreBoard,
    #[to = ""]
    Home
}