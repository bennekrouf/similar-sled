use crate::domain::coran::models::{VerseOutput, SimilarOutputAdapted};
use crate::models::Database;
use super::similars_by_key;
use log::info;

pub fn get(dbs: &Database, chapter: u32) -> Vec<SimilarOutputAdapted> {
    let chapter_key = chapter.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_key);

    let mut similar_objects: Vec<SimilarOutputAdapted> = Vec::new();

    for similar_key in similar_keys {
        let similar = similars_by_key::get(dbs, &similar_key);
        if similar.is_empty() {
            info!("similar empty for key : {:?}", &similar_key);
        } else {
            let mut verses: Vec<VerseOutput> = Vec::new();
            let mut similars: Vec<VerseOutput> = Vec::new();
            let kalima = similar[0].kalima.clone();

            for verse_output in similar[0].verses.iter().cloned() {
                if verse_output.chapter == chapter {
                    verses.push(verse_output);
                } else {
                    similars.push(verse_output);
                }
            }

            similar_objects.push(SimilarOutputAdapted {
                verses,
                similars,
                kalima,
            });
        }
    }

    similar_objects
}


fn get_similar_keys(dbs: &Database, chapter_key: &str) -> Vec<String> {
    let serialized_keys = dbs
        .chapter_similar_db
        .get(chapter_key.as_bytes())
        .unwrap_or(None)
        .unwrap_or_default();

    bincode::deserialize(&serialized_keys)
        .unwrap_or_default()
}