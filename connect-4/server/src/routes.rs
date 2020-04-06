use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::Mongoose;

use rocket::response::content;
use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};
use mongodb::coll::options;
use mongodb::oid;

use serde::{Serialize, Deserialize};
use serde_json::ser;
use bson::UtcDateTime;
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<ObjectId>,
    pub gameNumber: Option<String>,
    pub gameType: Option<String>,
    pub Player1Name: Option<String>,
    pub Player2Name: Option<String>,
    pub WinnerName: Option<String>,
    pub GameDate: UtcDateTime,
}

#[get("/games")]
pub fn get_games(conn: Mongoose) -> content::Json<String> {
    let coll = conn.collection("games");
    let mut cursor = coll
                    .find(Some(doc! {}), None)
                    .ok()
                    .expect("Failed to execute find.");
    let mut contents = Vec::new();
    
    for item in cursor {
        match item {
            Ok(doc) => if let Ok(d) = ser::to_string(&doc) {
                contents.push(d);
            },
            Err(e) => println!("{:?}", e)
        }
    }
    
    content::Json(format!("[{}]", contents.join(",")))

    // format!("HELLO? == {:?}", cursor.next().unwrap())
    // get request find Game() entry from MongoDB
    // handle error
    // post request and get entry and return it?
}

#[post("/games")]
pub fn create_game(conn: Mongoose) {
}

#[put("/games/<id>")]
pub fn update_game(conn: Mongoose, id: String) {

}

#[get("/games/<id>")]
pub fn get_game(conn: Mongoose, id: String) {

}

#[delete("/games/<id>")]
pub fn delete_game(conn: Mongoose, id: String) {

}
