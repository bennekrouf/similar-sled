use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::SimilarOutputAdapted;
use crate::models::Database;
use crate::domain::similar::similars_by_chapter;

#[get("/similars/<chapter_no>")]
pub fn get_verse_similar_by_chapter_route(
    dbs: State<Database>,
    chapter_no: u32,
) -> Json<Vec<SimilarOutputAdapted>>{
    let result = similars_by_chapter::get(&dbs, chapter_no);
    Json(result)
}