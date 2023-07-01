use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{SimilarOutput, Verse, VerseOutput, Database};
use crate::db::verse_by_chapter_and_ayat;
use crate::db::chapter_name;
use crate::utils::sort;

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let similars = get_similars_db(&dbs);
    Json(similars)
}

fn get_similars_db(dbs: &State<Database>) -> Vec<SimilarOutput> {
    let mut similars = dbs
        .similar_db
        .iter()
        .map(|result| {
            let (similar_key, similar_value) = result.expect("Failed to retrieve similar");
            let kalima = String::from_utf8_lossy(&similar_key).into_owned();
            let references: Vec<String> =
                bincode::deserialize(&similar_value).expect("Failed to deserialize references");

            let verses: Vec<VerseOutput> = references
                .iter()
                .map(|reference| {
                    let split: Vec<&str> = reference.split(":").collect();
                    let chapter: u32 = split[0].parse().expect("Not a valid u32");
                    let ayat: u32 = split[1].parse().expect("Not a valid u32");

                    let chapter_name_result = chapter_name::get(&dbs, chapter as u8);
                    let chapter_name = match chapter_name_result {
                        Ok(Some(name)) => name,
                        Ok(None) | Err(_) => String::from("Default Chapter"),
                    };

                    let text = match verse_by_chapter_and_ayat::get(&dbs, chapter, ayat) {
                        Ok(Some(verse_text)) => verse_text,
                        Ok(None) => String::from("Verse not found 1"),
                        Err(_) => String::from("Error retrieving verse 1"),
                    };

                    VerseOutput {
                        sourate: chapter_name,
                        verse: Verse {
                            text,
                            ayat,
                            chapter,
                        },
                    }
                })
                .collect();

            SimilarOutput { kalima, verses }
        })
        .collect::<Vec<SimilarOutput>>();

    sort::sort_similars(&mut similars);
    similars
}