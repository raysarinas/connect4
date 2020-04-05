#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() {

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
        index//,games
    ]
}

fn main() {
    let routes = get_routes();
    rocket::ignite().mount("/", routes).launch();
}