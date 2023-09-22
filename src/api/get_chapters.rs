use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::models::Chapter;
use crate::models::Database;
use crate::domain::similar::similars_by_chapter;
use crate::domain::verse::count_verses_in_chapter::count_verses_in_chapter;

#[get("/chapters")]
pub fn get_chapters(dbs: State<Database>) -> Json<Vec<Chapter>> {
    let chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| match result {
            Ok((_, value)) => {
                let chapter: Chapter = bincode::deserialize(&value).unwrap();
                let similar_objects = similars_by_chapter::get(&dbs, chapter.no as u32);
                let count = similar_objects.len() as u32;

                // Count ayat
                let count_ayat = count_verses_in_chapter(&dbs, chapter.no as u32);
                if count > 0 {
                    Some(Chapter { count_ayat: Some(count_ayat.unwrap()), count: Some(count), ..chapter })
                } else {
                    None
                }
            },
            _ => None,
        })
        .collect();

    Json(chapters)
}