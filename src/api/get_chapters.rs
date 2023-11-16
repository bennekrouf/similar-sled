use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::utils::parse_ranges::parse_ranges;
use crate::models::Chapter;
use crate::models::Database;
use crate::domain::similar::similars_by_chapter;
use crate::domain::verse::count_verses_in_chapter::count_verses_in_chapter;

#[get("/chapters?<ranges>")]
pub fn get_chapters(dbs: State<Database>, ranges: Option<String>) -> Json<Vec<Chapter>> {
    // println!("Parsed Ranges get_chapters: {:?}", ranges);

    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r));
    let chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| {
            if let Ok((_, value)) = result {
                let chapter: Chapter = bincode::deserialize(&value).unwrap();

                // Compute counts
                let similar_objects = similars_by_chapter::get(&dbs, chapter.no as u32, &parsed_ranges);
                let count = similar_objects.len() as u32;
                let count_ayat = count_verses_in_chapter(&dbs, chapter.no as u32);

                // Skip chapters with count_ayat of 0 or where count_ayat could not be computed
                if count_ayat.is_err() || count_ayat.as_ref().ok() == Some(&0) {
                    return None;
                }

                // Check if ranges parameter is provided and filter accordingly
                if let Some(range_str) = &ranges {
                    let range_tuples = parse_ranges(range_str);
                    if range_tuples.iter().any(|&(start, end)| chapter.no >= start && chapter.no <= end) {
                        Some(Chapter { count_ayat: count_ayat.ok(), count: Some(count), ..chapter })
                    } else {
                        None
                    }
                } else {
                    // If no ranges provided, do not filter and return all chapters
                    Some(Chapter { count_ayat: count_ayat.ok(), count: Some(count), ..chapter })
                }
            } else {
                None
            }
        })
        .collect();

    Json(chapters)
}
