#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use bson::{Bson, doc, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, error::Error};

pub fn connect_to_db() -> Result<mongodb::Client, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    client_options.app_name = Some("Connect4".to_string());

    // Get a handle to the deployment.
    // Create the client
    Client::with_options(client_options)
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/games")]
fn games() -> String {
    // format!("HELLO?")
    let client = connect_to_db().unwrap();
    let db = client.database("Connect4DB");
    let coll = db.collection("games");
    let mut cursor = coll
                    .find(Some(doc! {}), None)
                    .ok()
                    .expect("Failed to execute find.");

    let item = cursor.next();
    match item {
        Some(Ok(doc)) => {
          println!("{:?}", doc);
          format!("{:?}", doc)
        }
        _ => format!("Uh..."),
      }
    // get request find Game() entry from MongoDB
    // handle error
    // post request and get entry and return it?
}

#[post("/games")]
fn create_game() {
}

#[put("/games/<id>")]
fn update_game(id: String) {

}

#[get("/games/<id>")]
fn get_game(id: String) {

}

#[delete("/games/<id>")]
fn delete_game(id: String) {

}

fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
        files, games
    ]
}

fn main() {
    let routes = get_routes();
    rocket::ignite().mount("/", routes).launch();
}