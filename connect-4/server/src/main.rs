#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/games")]
fn games() -> &'static str {
    "Hello, world!"
    // get request find Game() entry from MongoDB
    // handle error
    // post request and get entry and return it?
}

#[post("/games")]
fn create_game() {
}

#[put("/games/<id>")]
fn update_game(id: usize) {

}

#[get("/games/<id>")]
fn get_game(id: usize) {

}

#[delete("/games/<id>")]
fn delete_game(id: usize) {

}

fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
        files//,games
    ]
}

fn main() {
    let routes = get_routes();
    rocket::ignite().mount("/", routes).launch();
}