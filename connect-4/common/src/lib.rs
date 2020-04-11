use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub gameNumber: String,
    pub gameType: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    // pub GameDate: <T>,//DateTime<Utc>,
    #[serde(skip)] // FIX LATER UGHHH
    pub GameDate: String,//&'a str,
}