use rocket::{get, State};
use crate::models::Database;
// use crate::utils::count;
use rocket_contrib::json::Json;

#[get("/count")]
pub fn get(dbs: State<Database>) -> Json<i32> {
   Json(5)
}