use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::xhr_guard::XHR;
use crate::models::{Database, Exercise};
use crate::domain::exercise::get_exercises::get_exercises;
use crate::utils::parse_ranges::parse_ranges;

#[get("/exercise_list?<ranges>")]
pub fn generate_exercise_list_endpoint(ranges: Option<String>, dbs: State<Database>, _xhr: XHR) -> Json<Vec<Exercise>> {
    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r));
    let exercises = get_exercises(&dbs, &parsed_ranges);
    Json(exercises)
}
