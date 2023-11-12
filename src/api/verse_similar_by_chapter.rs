use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::SimilarOutputAdapted;
use crate::models::Database;
use crate::domain::similar::similars_by_chapter;
use crate::utils::parse_ranges::parse_ranges;

#[get("/similars/<chapter_no>?<ranges>")]
pub fn get_verse_similar_by_chapter_route(
    dbs: State<Database>,
    chapter_no: u32,
    ranges: Option<String>,
) -> Json<Vec<SimilarOutputAdapted>>{
    println!("Parsed Ranges get_verse_similar_by_chapter_route: {:?}", ranges);

    // Parse the range parameter into an Option<Vec<(u8, u8)>>
    let chapter_range = ranges.map(|s| parse_ranges(&s));
    
    let result: Vec<SimilarOutputAdapted> = similars_by_chapter::get(&dbs, chapter_no, chapter_range);
    Json(result)
}