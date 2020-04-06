#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
// use bson::{Bson, doc, oid::ObjectId};
// use mongodb::{options::ClientOptions, Client, error::Error};

mod routes;

use rocket_contrib::databases::mongodb;

#[database("mongoose")]
pub struct Mongoose(mongodb::db::Database);

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
    format!("HELLO?")
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
    rocket::ignite()
            .attach(Mongoose::fairing())
            .mount("/", routes)
            .launch();
}