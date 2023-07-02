use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{SimilarOutput, Database};
use crate::db::similars_all;

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let similars = similars_all::get_similars_db(&dbs);
    Json(similars)
}