use rocket::{get, State};
use crate::utils;
use rocket_contrib::json::Json;
use crate::models::Database;

#[get("/similar/<chapter_no>")]
pub fn get_similar(chapter_no: u8, dbs: State<Database>) -> Json<Vec<(String, Vec<(String, String, u32)>)>> {
   // Retrieve the list of similars from the similar_db
    let similars: Vec<(String, Vec<(String, String, u32)>)> = dbs.similar_db
        .iter()
        .map(|result| {
            let (key, value) = result.expect("Failed to retrieve similar");
            let key_string = String::from_utf8_lossy(&key).into_owned();
            let references: Vec<(u32, u32)> =
                bincode::deserialize(&value).expect("Failed to deserialize references");

            let updated_references: Vec<(String, String, u32)> = references
                .iter()
                // .filter(|&&reference| reference.0 == u32::from(chapter_no)) // Filter by chapter_no
                .map(|&reference| {
                    let ayat = reference.1;
                    println!("{} {}", reference.0, reference.1);
                    let chapter_name = utils::get_chapter_name(&dbs.chapter_db, reference.0 as u8)
                     .unwrap_or_else(|_| Some(String::from("Default Chapter")))
                    .unwrap();

                    let verse_text = match utils::get_verse_by_chapter_and_ayat(&dbs.verse_db, reference.0, ayat) {
                        Ok(Some(text)) => text,
                        Ok(None) => String::from("Verse not found"),
                        Err(_) => String::from("Error retrieving verse"),
                    };

                    (verse_text, chapter_name, ayat)
            })
            .collect::<Vec<(String, String, u32)>>();

            (key_string, updated_references)
        })
        .collect();

    // Return the similars as JSON
    Json(similars)
}


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

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<(String, Vec<(String, String, u32)>)>> {
   // Retrieve the list of similars from the similar_db
    let similars: Vec<(String, Vec<(String, String, u32)>)> = dbs.similar_db
        .iter()
        .map(|result| {
            let (key, value) = result.expect("Failed to retrieve similar");
            let key_string = String::from_utf8_lossy(&key).into_owned();
            let references: Vec<(u32, u32)> =
                bincode::deserialize(&value).expect("Failed to deserialize references");

            let updated_references: Vec<(String, String, u32)> = references
                .iter()
                .map(|&reference| {
                    let ayat = reference.1;
                    println!("{} {}", reference.0, reference.1);
                    let chapter_name = utils::get_chapter_name(&dbs.chapter_db, reference.0 as u8)
                     .unwrap_or_else(|_| Some(String::from("Default Chapter")))
                    .unwrap();

                    let verse_text = match utils::get_verse_by_chapter_and_ayat(&dbs.verse_db, reference.0, ayat) {
                        Ok(Some(text)) => text,
                        Ok(None) => String::from("Verse not found"),
                        Err(_) => String::from("Error retrieving verse"),
                    };

                    (verse_text, chapter_name, ayat)
            })
            .collect::<Vec<(String, String, u32)>>();

            (key_string, updated_references)
        })
        .collect();

    // Return the similars as JSON
    Json(similars)
}
