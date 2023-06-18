use rocket::{get, routes, Rocket, State};
use crate::utils;
use rocket_contrib::json::Json;
use crate::models::Database;

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

#[get("/similar/<chapter_no>")]
pub fn get_similar(chapter_no: u8, dbs: State<Database>) -> Json<Vec<(String, Vec<(u32, u32)>)>> {
   // Retrieve the list of similars from the similar_db
    let similars: Vec<(String, Vec<(u32, u32)>)> = dbs.similar_db
        .iter()
        .map(|result| {
            let (key, value) = result.expect("Failed to retrieve similar");
            let key_string = String::from_utf8_lossy(&key).into_owned();
            let references: Vec<(u32, u32)> =
                bincode::deserialize(&value).expect("Failed to deserialize references");
            (key_string, references)
        })
        .collect();

    // Return the similars as JSON
    Json(similars)
}
