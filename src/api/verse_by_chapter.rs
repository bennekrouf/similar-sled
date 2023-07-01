use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::Database;
use crate::db::chapter_name;
use crate::db::verses_by_chapter;

#[get("/verse/<chapter_no>")]
pub fn get_verse(chapter_no: u8, dbs: State<Database>) -> Json<serde_json::Value> {
    let chapter = chapter_name::get(&dbs, chapter_no).unwrap();
    let verse = verses_by_chapter::get(&dbs, chapter_no).unwrap();

    // Create a JSON value using serde_json
    let json_value = serde_json::json!({
        "chapter": chapter,
        "verse": verse,
    });

    // Wrap the JSON value in a `Json` struct
    Json(json_value)
}