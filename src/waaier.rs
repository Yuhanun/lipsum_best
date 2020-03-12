use rocket::*;
use serde_json::*;

use crate::db;

#[post("/waaier/enroll/<name>")]
pub fn enroll(conn: db::LipsumDb, name: String) -> String {
    match db::insert_food_entry_today(&conn, &name) {
        Ok(entry) => json!({
            "status": false,
            "payload": {
                "entry": {
                    "name": entry.person_name,
                    "food_date": format!("{}", entry.food_date),
                    "id": entry.id
                }
            }
        })
        .to_string(),
        Err(e) => json!({
            "status": true,
            "payload": {
                "error": format!("{}", e)
            }
        })
        .to_string(),
    }
}
