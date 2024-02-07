use rocket::{get, State};
use rocket::serde::{json::Json};

use crate::utils::parse_ranges::parse_ranges;
use crate::models::chapter::Chapter;
use crate::models::database::Database;
use crate::domain::similar::similars_by_chapter::similars_by_chapter;
use crate::domain::verse::count_verses_in_chapter::count_verses_in_chapter;
use crate::utils::read_labels::read_labels;

#[get("/chapters?<ranges>")]
pub fn get_chapters(dbs: &State<Database>, ranges: Option<String>) -> Json<Vec<Chapter>> {
    let default_range = read_labels().unwrap_or((1, 114));
    let parsed_ranges = match ranges.as_deref() {
        Some("undefined") | None => Some(parse_ranges(&format!("{}-{}", default_range.0, default_range.1))),
        Some(r) => Some(parse_ranges(r)),
    };
    println!("Parsed Ranges get_chapters: {:?}", parsed_ranges);

    let chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| {
            if let Ok((_, value)) = result {
                let chapter: Chapter = bincode::deserialize(&value).unwrap();

                // Compute counts
                let similar_objects = similars_by_chapter(&dbs, chapter.no as u32, &parsed_ranges);
                let count = similar_objects.len() as u32;
                let count_ayat = count_verses_in_chapter(&dbs, chapter.no as u32);

                // Skip chapters with count_ayat of 0 or where count_ayat could not be computed
                if count_ayat.is_err() || count_ayat.as_ref().ok() == Some(&0) {
                    return None;
                }

                // Check if ranges parameter is provided and filter accordingly
                if parsed_ranges.is_some() && parsed_ranges.as_ref().unwrap().iter().any(|&(start, end)| chapter.no >= start && chapter.no <= end) {
                    Some(Chapter { count_ayat: count_ayat.ok(), count: Some(count), ..chapter })
                } else if parsed_ranges.is_none() {
                    // If no ranges provided, do not filter and return all chapters
                    Some(Chapter { count_ayat: count_ayat.ok(), count: Some(count), ..chapter })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Json(chapters)
}
