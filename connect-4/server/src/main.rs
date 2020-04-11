#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::databases::mongodb;

pub mod routes;

#[database("mongoose")]
pub struct Mongoose(mongodb::db::Database);

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
        files,
        routes::get_all_games,
        routes::create_game,
        routes::get_game,
        routes::update_game,
        routes::delete_game
    ]
}

fn main() {
    let routes = get_routes();
    rocket::ignite()
            .attach(Mongoose::fairing())
            .mount("/", routes)
            .launch();
}