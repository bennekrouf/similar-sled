use rocket::serde::json::Json;
use rocket::{get, State};

use crate::db::exercise::get_solution;
use crate::models::Database;
use crate::models::ExerciseOutput;

#[get("/solutions/<kalima>")]
pub fn get_solutions(kalima: String, dbs: &State<Database>) -> Json<Vec<ExerciseOutput>> {
    let solutions = get_solution::get_solution(dbs, &kalima);
    Json(solutions)
}

