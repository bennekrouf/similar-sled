use crate::models::{Verse, Database, VerseOutput, SimilarOutput};
use crate::utils::verse_by_chapter_and_ayat;
use crate::utils::chapter_name;
use crate::utils::sort;

pub fn get_similars_core(dbs: &Database) -> Vec<SimilarOutput> {
    let mut similars = dbs
        .similar_db
        .iter()
        .map(|result| {
            let (similar_key, similar_value) = result.expect("Failed to retrieve similar");
            let kalima = String::from_utf8_lossy(&similar_key).into_owned();
            let references: Vec<(u32, u32)> =
                bincode::deserialize(&similar_value).expect("Failed to deserialize references");

            let verses: Vec<VerseOutput> = references
                .iter()
                .map(|&reference| {
                    let chapter = reference.0;
                    let ayat = reference.1;
                    let chapter_name = chapter_name::get(&dbs.chapter_db, chapter as u8)
                        .unwrap_or_else(|_| Some(String::from("Default Chapter")))
                        .unwrap();

                    let text = match verse_by_chapter_and_ayat::get(
                        &dbs.verse_db,
                        chapter,
                        ayat,
                    ) {
                        Ok(Some(verse_text)) => verse_text,
                        Ok(None) => String::from("Verse not found"),
                        Err(_) => String::from("Error retrieving verse"),
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

            SimilarOutput {
                kalima,
                verses,
            }
        })
        .collect::<Vec<SimilarOutput>>();

    sort::sort_similars(&mut similars);
    similars
}