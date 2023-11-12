use super::similars_by_key;

use crate::models::{VerseOutput, SimilarOutputAdapted};
use crate::models::Database;

pub fn get(dbs: &Database, chapter_no: u32, chapter_range: Option<Vec<(u8, u8)>>) -> Vec<SimilarOutputAdapted> {
    let chapter_key = chapter_no.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_key);

    let mut similar_objects: Vec<SimilarOutputAdapted> = Vec::new();

    for kalima in similar_keys {
        let similar = similars_by_key::get(dbs, &kalima);

        // Filter logic
        if !similar.is_empty() {
            let mut verses: Vec<VerseOutput> = Vec::new();
            let mut similars: Vec<VerseOutput> = Vec::new();
            let mut opposites: Vec<VerseOutput> = Vec::new();
            let kalima = similar[0].kalima.clone();

            for verse_output in &similar[0].verses {
                if let Some(ref ranges) = chapter_range {
                    if verse_belongs_to_range(verse_output, ranges) {
                        verses.push(verse_output.clone());
                    }
                } else {
                    verses.push(verse_output.clone());
                }

                if verse_output.chapter_no == chapter_no {
                    similars.push(verse_output.clone());
                }
            }

            if let Some(opposite_verses) = &similar[0].opposites {
                for verse_output in opposite_verses {
                    if chapter_range.is_none() || verse_belongs_to_range(verse_output, chapter_range.as_ref().unwrap()) {
                        opposites.push(verse_output.clone());
                    }
                }
            }

            if !verses.is_empty() && (!similars.is_empty() || !opposites.is_empty()) {
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

fn verse_belongs_to_range(verse: &VerseOutput, range: &[(u8, u8)]) -> bool {
    range.iter().any(|&(start, end)| verse.chapter_no >= start as u32 && verse.chapter_no <= end as u32)
}