#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

mod db;
mod models;
mod schema;
mod waaier;

fn main() {
    rocket::ignite()
        .attach(db::LipsumDbConn::fairing())
        .mount("/api", rocket::routes![waaier::enroll])
        .launch();
}
