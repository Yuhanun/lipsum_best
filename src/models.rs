use crate::schema::foodentries;
#[table_name = "foodentries"]

#[derive(Insertable, Debug)]
pub struct NewFoodEntry<'a> {
    pub person_name: &'a str,
}

#[derive(Queryable, Debug)]
pub struct FoodEntry {
    pub id: i32,
    pub person_name: String,
    pub food_date: chrono::NaiveDateTime,
}
