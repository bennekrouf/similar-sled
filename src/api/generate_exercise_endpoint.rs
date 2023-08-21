use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::xhr_guard::XHR;
use crate::models::{VerseUngrouped, Database, ExerciseType, ChapterAyah};
use crate::db::exercise::find_discriminant::{self, generate_exercises};

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(kalima: String, dbs: State<Database>, _xhr: XHR) -> Option<Json<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)>> {
   find_discriminant::generate_one(&dbs, kalima, ExerciseType::B).map(Json)
}

#[get("/exercise_list/<kalima>")]
pub fn generate_exercise_list_endpoint(kalima: String, dbs: State<Database>, _xhr: XHR) -> Json<Vec<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)>> {
    let exercises = generate_exercises(&dbs, &kalima);
    Json(exercises)
}
