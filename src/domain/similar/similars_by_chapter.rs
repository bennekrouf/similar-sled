use log::info;

use super::similars_by_key;

use crate::models::{VerseOutput, SimilarOutputAdapted};
use crate::models::Database;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;

pub fn get(dbs: &Database, chapter_no: u32) -> Vec<SimilarOutputAdapted> {
    let chapter_key = chapter_no.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_key);

    let mut similar_objects: Vec<SimilarOutputAdapted> = Vec::new();

    for kalima in similar_keys {
        let similar = similars_by_key::get(dbs, &kalima);
        if similar.is_empty() {
            info!("similar empty for key : {:?}", &kalima);
        } else {
            let mut verses: Vec<VerseOutput> = Vec::new();
            let mut similars: Vec<VerseOutput> = Vec::new();
            let mut opposites: Vec<VerseOutput> = Vec::new();
            let kalima = similar[0].kalima.clone();

            for mut verse_output in similar[0].verses.iter().cloned() {
                verse_output.sourate = Some(sourate_name_from_verse(dbs, &verse_output));
                if verse_output.chapter_no == chapter_no {
                    verses.push(verse_output);
                } else {
                    similars.push(verse_output);
                }
            }

            if let Some(opposite_verses) = &similar[0].opposites {
                for mut verse_output in opposite_verses.iter().cloned() {
                    verse_output.sourate = Some(sourate_name_from_verse(dbs, &verse_output));
                    opposites.push(verse_output);
                }
            }

            similar_objects.push(SimilarOutputAdapted {
                verses,
                similars,
                opposites,
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