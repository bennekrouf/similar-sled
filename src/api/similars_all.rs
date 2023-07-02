use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{SimilarOutput, Database, Chapter};
use crate::db::similars_all;

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let similars = similars_all::get_similars_db(&dbs);
    Json(similars)
}

#[get("/chapters")]
pub fn get_chapters(dbs: State<Database>) -> Json<Vec<Chapter>> {
    let chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| match result {
            Ok((_, value)) => Some(bincode::deserialize(&value).unwrap()),
            _ => None,
        })
        .collect();

    Json(chapters)
}