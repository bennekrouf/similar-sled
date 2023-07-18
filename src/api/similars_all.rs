use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{SimilarOutput, Database, Chapter};
use crate::db::similar::similars_all;
use crate::db::similar::similars_by_chapter;

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let similars = similars_all::get_similars_db(&dbs);
    Json(similars)
}

#[get("/chapters")]
pub fn get_chapters(dbs: State<Database>) -> Json<Vec<Chapter>> {
    let mut chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| match result {
            Ok((_, value)) => {
                let chapter: Chapter = bincode::deserialize(&value).unwrap();
                let similar_objects = similars_by_chapter::get_chapter_similars(&dbs, chapter.no as u32);
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

    // Sort chapters based on the count field in ascending order
    // chapters.sort_by_key(|chapter| chapter.count.unwrap_or(0));

    Json(chapters)
}