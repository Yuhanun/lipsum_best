use crate::diesel::prelude::*;
use crate::models::*;
use crate::schema::foodentries::dsl::*;
use chrono::Datelike;
use diesel::pg::PgConnection;
use rocket_contrib::database;

#[database("lipsum_best")]
pub struct LipsumDb(rocket_contrib::databases::diesel::PgConnection);

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

pub fn insert_food_entry_today(conn: &PgConnection, name: &str) -> QueryResult<FoodEntry> {
    diesel::insert_into(foodentries)
        .values(&NewFoodEntry { person_name: name })
        .get_result(conn)
}

pub fn get_all_food_entries(conn: &PgConnection) -> Result<Vec<FoodEntry>, diesel::result::Error> {
    foodentries.load::<FoodEntry>(conn)
}

pub fn get_today_food_entries(
    conn: &PgConnection,
) -> Result<Vec<FoodEntry>, diesel::result::Error> {
    let today = chrono::Utc::now().naive_local();
    Ok(get_all_food_entries(conn)?
        .into_iter()
        .filter(|elem| {
            elem.food_date.year() == today.year()
                && elem.food_date.month() == today.month()
                && elem.food_date.day() == today.day()
        })
        .collect::<Vec<FoodEntry>>())
}

// fn main() {
//     let conn = establish_connection();

//     let new_entry = NewFoodEntry {
//         person_name: String::from("John"),
//     };

//     let result: FoodEntry = diesel::insert_into(foodentries)
//         .values(&new_entry)
//         .get_result(&conn)
//         .unwrap();

//     let results = foodentries
//         .load::<FoodEntry>(&conn)
//         .expect("Error loading posts");

//     println!("{:#?}", results);
// }
