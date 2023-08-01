use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::domain::coran::models::{SimilarOutput, Chapter};
use crate::models::Database;
use crate::db::similar::similars_all;
use crate::db::similar::similars_by_chapter;

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let similars = similars_all::get(&dbs);
    Json(similars)
}

#[get("/chapters")]
pub fn get_chapters(dbs: State<Database>) -> Json<Vec<Chapter>> {
    let chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| match result {
            Ok((_, value)) => {
                let chapter: Chapter = bincode::deserialize(&value).unwrap();
                let similar_objects = similars_by_chapter::get(&dbs, chapter.no as u32);
                let count = similar_objects.len() as u32;
                if count > 0 {
                    Some(Chapter { count: Some(count), ..chapter })
                } else {
                    None
                }
            },
            _ => None,
        })
        .collect();

    Json(chapters)
}