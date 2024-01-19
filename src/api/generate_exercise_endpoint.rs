use rocket::{get, State};
use rocket::serde::json::Json;

use crate::models::{Database, Exercise};
use crate::domain::exercise::get_exercises::get_exercises;
use crate::utils::parse_ranges::parse_ranges;

#[get("/exercise_list?<ranges>")]
pub fn generate_exercise_list_endpoint(
    dbs: &State<Database>,
    ranges: Option<String> ,
) -> Json<Vec<Exercise>> {
    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r));
    let exercises = get_exercises(&dbs, &parsed_ranges);
    Json(exercises)
}
