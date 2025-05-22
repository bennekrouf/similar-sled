use rocket::serde::json::Json;
use rocket::{get, State};

use crate::db::chapter::chapter_name;
use crate::db::verse::verses_by_chapter;
use crate::models::Database;

#[get("/verse/<chapter_no>")]
pub fn get_verse(chapter_no: u8, dbs: &State<Database>) -> Json<serde_json::Value> {
    let chapter = chapter_name::get(dbs, chapter_no).unwrap();
    let verse = verses_by_chapter::get(dbs, chapter_no).unwrap();

    let json_value = serde_json::json!({
        "chapter": chapter,
        "verse": verse,
    });

    Json(json_value)
}
