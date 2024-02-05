use rocket::{post, State};
use rocket::serde::json::Json;

use crate::models::{Database, Exercise};
use crate::domain::exercise::get_exercises::get_exercises;
use crate::utils::parse_ranges::parse_ranges;

use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig,
    // analytic::Analytic,
};

#[post("/exercises?<ranges>", format = "json", data = "<user_stats>")]
pub fn generate_exercises_endpoint(
    dbs: &State<Database>,
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>, 
    ranges: Option<String>,
) -> Json<Vec<Exercise>> {
    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r));
    let exercises = get_exercises(&dbs, &parsed_ranges);
    Json(exercises)
}