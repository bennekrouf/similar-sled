use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::xhr_guard::XHR;
use crate::models::{Database, Exercise};
use crate::domain::exercise::get_exercises::get_exercises;

#[get("/exercise_list/<kalima>")]
pub fn generate_exercise_list_endpoint(kalima: String, dbs: State<Database>, _xhr: XHR) -> Json<Vec<Exercise>> {
    let exercises = get_exercises(&dbs, &kalima);
    Json(exercises)
}
