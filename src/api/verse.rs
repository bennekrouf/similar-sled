use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::Database;
use crate::utils;

#[get("/verse/<chapter_no>")]
pub fn get_verse(chapter_no: u8, dbs: State<Database>) -> Json<serde_json::Value> {
    let chapter = utils::get_chapter_name(&dbs.chapter_db, chapter_no).unwrap();
    let verse = utils::get_verses_by_chapter(&dbs.verse_db, chapter_no).unwrap();

    // Create a JSON value using serde_json
    let json_value = serde_json::json!({
        "chapter": chapter,
        "verse": verse,
    });

    // Wrap the JSON value in a `Json` struct
    Json(json_value)
}