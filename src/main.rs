#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate db_handler;

#[get("/")]
fn welcome() -> String {
    format!("Welcome to lipsum.best!")
}

fn main() {
    let connection = db_handler::establish_connection();
    println!("{:?}", connection);
    // rocket::ignite().mount("/", routes![welcome]).launch();
}
