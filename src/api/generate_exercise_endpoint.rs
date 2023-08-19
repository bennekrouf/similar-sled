use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::models::{VerseUngrouped, Database, ExerciseType};
use crate::db::exercise::find_discriminant::{self, generate_exercises};

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(kalima: String, dbs: State<Database>) -> Option<Json<(VerseUngrouped, Vec<String>, ExerciseType)>> {
   find_discriminant::generate_one(&dbs, kalima, ExerciseType::B).map(Json)
}

#[get("/exercise_list/<kalima>")]
pub fn generate_exercise_list_endpoint(kalima: String, dbs: State<Database>) -> Json<Vec<(VerseUngrouped, Vec<String>, ExerciseType)>> {
    let exercises = generate_exercises(&dbs, &kalima);
    Json(exercises)
}
