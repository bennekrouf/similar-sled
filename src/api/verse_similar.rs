use std::collections::HashMap;
use rocket::{get, State};
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;

use crate::models::{ChapterSimilarOutput, Verse, Database};

#[get("/verse_similars/<chapter>")]
pub fn get_chapter_similars_route(
    dbs: State<Database>,
    chapter: u32,
) -> Result<Json<ChapterSimilarOutput>, NotFound<String>> {
    match get_chapter_similars(&dbs, chapter) {
        Some(chapter_similars) => Ok(Json(chapter_similars)),
        None => Err(NotFound(format!("Chapter {} not found", chapter))),
    }
}

fn get_chapter_similars(dbs: &Database, chapter: u32) -> Option<ChapterSimilarOutput> {
    let chapter_key = chapter.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_key);

    if similar_keys.is_empty() {
        None
    } else {
        Some(ChapterSimilarOutput {
            chapter,
            similar_keys,
        })
    }
}

fn get_similar_keys(dbs: &Database, key: &str) -> Vec<String> {
    let serialized_keys = dbs
        .verse_similar_db
        .get(key.as_bytes())
        .unwrap_or(None)
        .unwrap_or_default();

    bincode::deserialize(&serialized_keys)
        .unwrap_or_default()
}

#[get("/verse_similars_all")]
pub fn get_verse_similars_all_route(dbs: State<Database>) -> Json<Vec<ChapterSimilarOutput>> {
    let verse_similars = get_verse_similars(&dbs);
    Json(verse_similars)
}

fn get_verse_similars(dbs: &Database) -> Vec<ChapterSimilarOutput> {
    let mut chapter_similars = Vec::new();
    let mut chapter_map: HashMap<u32, Vec<String>> = HashMap::new();

    for result in dbs.verse_similar_db.iter() {
        match result {
            Ok((verse_key, serialized_keys)) => {
                let verse = parse_verse_key(&verse_key);
                let similar_keys: Vec<String> = bincode::deserialize(&serialized_keys)
                    .expect("Failed to deserialize similar keys");

                let chapter = verse.chapter;
                let chapter_entry = chapter_map.entry(chapter).or_insert_with(Vec::new);
                chapter_entry.extend(similar_keys);
            }
            Err(error) => {
                // Handle the error as per your requirement
                eprintln!("Error while iterating verse_similar_db: {:?}", error);
            }
        }
    }

    for (chapter, similar_keys) in chapter_map {
        chapter_similars.push(ChapterSimilarOutput {
            chapter,
            similar_keys,
        });
    }

    chapter_similars
}


fn parse_verse_key(verse_key: &[u8]) -> Verse {
    let key_str = String::from_utf8_lossy(verse_key);
    let parts: Vec<&str> = key_str.split(":").collect();
    let chapter = parts[0].parse().expect("Invalid chapter number");
    let ayat = parts[1].parse().expect("Invalid ayat number");

    Verse { chapter, ayat, text: String::new() }
}