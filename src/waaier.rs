use rocket::*;
use serde_json::json;


#[get("/waaier/enroll/<name>")]
pub fn hello(
    state: State<std::sync::Arc<std::sync::Mutex<diesel::PgConnection>>>,
    name: String,
) -> String {
    db::insert_food_entry_today(&state.get().lock(), name: &str)
    json!({
        "status": false,
        "payload": {}
    }).to_string()
}
