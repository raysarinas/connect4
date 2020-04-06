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
pub struct Game {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<ObjectId>,
    pub gameNumber: Option<String>,
    pub gameType: Option<String>,
    pub Player1Name: Option<String>,
    pub Player2Name: Option<String>,
    pub WinnerName: Option<String>,
    pub GameDate: Option<String>,
}

#[get("/games")]
pub fn get_all_games(conn: Mongoose) -> content::Json<String> {
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
}

#[post("/games", data = "<game>")]
pub fn create_game(conn: Mongoose, game: Json<Game>) -> content::Json<String> {
    let inner = game.into_inner();
    let doc = doc! {
        "gameNumber": inner.gameNumber.unwrap(),
        "gameType": inner.gameType.unwrap(),
        "Player1Name": inner.Player1Name.unwrap(),
        "Player2Name": inner.Player2Name.unwrap(),
        "WinnerName": inner.WinnerName.unwrap(),
        "GameDate": inner.GameDate.unwrap(),
    };
    let test = doc.clone();
    let coll = conn.collection("games");
    let mut cursor = coll
                    .insert_one(doc, None);
    
    content::Json(format!("INSERTED THE DOC = {:?}", test))
}

#[put("/games/<id>", data = "<game>")]
pub fn update_game(conn: Mongoose, id: String, game: Json<Game>) -> content::Json<String> {
    let inner = game.into_inner();
    let coll = conn.collection("games");

    let filter = doc!{ "_id": bson::oid::ObjectId::with_string(&id).unwrap()};
    let update = doc!{"$set" => {"WinnerName" => inner.WinnerName.unwrap()}};

    coll.update_one(filter.clone(), update, None).unwrap();

    content::Json(format!("{:?}", filter))
}

#[get("/games/<id>")]
pub fn get_game(conn: Mongoose, id: String) -> content::Json<String> {
    let coll = conn.collection("games");

    let mut gg = coll.find_one(Some(doc! { "_id": bson::oid::ObjectId::with_string(&id).unwrap() }), None)
                    .expect("Document not found");

    content::Json(format!("{:?}", gg))
}

#[delete("/games/<id>")]
pub fn delete_game(conn: Mongoose, id: String) -> content::Json<String> {
    // let inner = game.into_inner();
    let coll = conn.collection("games");

    let filter = doc!{ "_id": bson::oid::ObjectId::with_string(&id).unwrap()};

    coll.delete_one(filter.clone(), None).unwrap();

    content::Json(format!("{:?}", filter))
}
