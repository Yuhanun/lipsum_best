#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde_json;

mod database;
mod models;
mod schema;
mod waaier;
use database as db;

fn main() {
    rocket::ignite()
        .manage(std::sync::Arc::from(std::sync::Mutex::from(
            db::establish_connection(),
        )))
        .mount("/api", rocket::routes![waaier::hello])
        .launch();
}
