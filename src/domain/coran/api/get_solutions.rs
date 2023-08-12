use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::domain::coran::models::ExerciseOutput;
use crate::models::Database;
use crate::domain::coran::db::exercise::get_solution;

#[get("/solutions/<kalima>")]
pub fn get_solutions(kalima: String, dbs: State<Database>) -> Json<Vec<ExerciseOutput>> {
    let solutions = get_solution::get_solution(&dbs, &kalima);
    Json(solutions)
}
