use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{Verse, Database};
use crate::utils::verse_by_chapter_and_ayat;
use crate::utils::chapter_name;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct VerseOutput {
    verse: Verse,
    sourate: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutput {
    verses: Vec<VerseOutput>,
    similar: String,
}

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let similars: Vec<SimilarOutput> = dbs
        .similar_db
        .iter()
        .map(|result| {
            let (key, value) = result.expect("Failed to retrieve similar");
            let key_string = String::from_utf8_lossy(&key).into_owned();
            // println!("Similar: {}", key_string);
            let references: Vec<(u32, u32)> =
                bincode::deserialize(&value).expect("Failed to deserialize references");

            let updated_references: Vec<VerseOutput> = references
                .iter()
                .map(|&reference| {
                    let ayat = reference.1;
                    let chapter_no = reference.0;
                    let chapter_name = chapter_name::get(&dbs.chapter_db, chapter_no as u8)
                        .unwrap_or_else(|_| Some(String::from("Default Chapter")))
                        .unwrap();

                    let verse_text = match verse_by_chapter_and_ayat::get(
                        &dbs.verse_db,
                        chapter_no,
                        ayat,
                    ) {
                        Ok(Some(text)) => text,
                        Ok(None) => String::from("Verse not found"),
                        Err(_) => String::from("Error retrieving verse"),
                    };

                    let the_verse =  Verse {
                        text: verse_text,
                        ayat: ayat,
                        chapter: chapter_no,
                    };

                    VerseOutput {
                        verse: the_verse,
                        sourate: chapter_name, 
                    }
                })
                .collect();

            SimilarOutput {
                similar: key_string,
                verses: updated_references,
            }
        })
        .collect();
    Json(similars)
}