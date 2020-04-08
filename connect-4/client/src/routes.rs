//! Routes by yew_router

pub mod howtoconnect4;
pub mod connect4comp;
pub mod home;

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/HowToConnect4"]
    HowToConnect4,
    #[to = "#/Connect4Computer"]
    Connect4Computer,
    #[to = ""]
    Home
}