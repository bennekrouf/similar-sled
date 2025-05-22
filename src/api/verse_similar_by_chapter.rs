use crate::db::similar::similars_by_chapter;
use crate::models::Database;
use crate::models::SimilarOutputAdapted;
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/similars/<chapter_no>")]
pub fn get_verse_similar_by_chapter_route(
    dbs: &State<Database>,
    chapter_no: u32,
) -> Json<Vec<SimilarOutputAdapted>> {
    let result = similars_by_chapter::get(dbs, chapter_no);
    Json(result)
}
