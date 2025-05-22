// src/api/generate_exercise_endpoint.rs
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::db::exercise::find_discriminant;
use crate::models::Database;
use crate::models::VerseUngrouped;

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(
    kalima: String,
    dbs: &State<Database>,
) -> Option<Json<(VerseUngrouped, Vec<String>)>> {
    find_discriminant::generate(dbs, kalima).map(Json)
}
