use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::domain::coran::{models::{SimilarOutputAdapted}};
use crate::models::Database;
use crate::db::similar::similars_by_chapter;

#[get("/similars/<chapter>")]
pub fn get_verse_similar_by_chapter_route(
    dbs: State<Database>,
    chapter: u32,
) -> Json<Vec<SimilarOutputAdapted>>{
    let result = similars_by_chapter::get_chapter_similars_adapted(&dbs, chapter);
    Json(result)
}