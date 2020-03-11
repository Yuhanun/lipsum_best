#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;
mod models;
mod schema;

use database as db;

fn main() {
    let conn = db::establish_connection();

    // println!("Insert: {:#?}", db::insert_food_entry_today(&conn, "John"));

    // println!("All: {:#?}", db::get_all_food_entries(&conn));
    // println!("Today: {:#?}", db::get_today_food_entries(&conn));
}
