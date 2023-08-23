use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::xhr_guard::XHR;
use crate::models::{Statement, Database, ExerciseType, Alternative};
use crate::db::exercise::generate::{self, generate_exercises};

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(kalima: String, dbs: State<Database>, _xhr: XHR) -> Option<Json<(Statement, Vec<Alternative>, ExerciseType)>> {
   generate::generate_one(&dbs, kalima, ExerciseType::B).map(Json)
}

#[get("/exercise_list/<kalima>")]
pub fn generate_exercise_list_endpoint(kalima: String, dbs: State<Database>, _xhr: XHR) -> Json<Vec<(Statement, Vec<Alternative>, ExerciseType)>> {
    let exercises = generate_exercises(&dbs, &kalima);
    Json(exercises)
}
