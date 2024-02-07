use super::similars_by_key;
use log::info;

use crate::models::{
    database::Database, verse_output::VerseOutput, similar_output_adapted::SimilarOutputAdapted
};
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::utils::is_chapter_in_range::is_chapter_in_range;

pub fn similars_by_chapter(dbs: &Database, chapter_no: u32, chapter_range: &Option<Vec<(u8, u8)>>) -> Vec<SimilarOutputAdapted> {
    // println!("Parsed Ranges: {:?}", chapter_range);
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
                if is_chapter_in_range(&verse_output.chapter_no, &chapter_range) {
                    verse_output.sourate = Some(sourate_name_from_verse(dbs, &verse_output));
                    if verse_output.chapter_no == chapter_no {
                        verses.push(verse_output);
                    } else {
                        similars.push(verse_output);
                    }
                }
            }

            if let Some(opposite_verses) = &similar[0].opposites {
                for mut verse_output in opposite_verses.iter().cloned() {
                    if is_chapter_in_range(&verse_output.chapter_no, &chapter_range) {
                        verse_output.sourate = Some(sourate_name_from_verse(dbs, &verse_output));
                        opposites.push(verse_output);
                    }
                }
            }

            if !similars.is_empty() || !opposites.is_empty() {
                similar_objects.push(SimilarOutputAdapted {
                    verses,
                    similars,
                    opposites,
                    kalima,
                });
            }
        }
    }

    // Sorting the vector by the total length of verses, similars, and opposites.
    similar_objects.sort_by(|a, b| {
        let len_a = a.verses.len() + a.similars.len() + a.opposites.len();
        let len_b = b.verses.len() + b.similars.len() + b.opposites.len();
        len_a.cmp(&len_b)
    });

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