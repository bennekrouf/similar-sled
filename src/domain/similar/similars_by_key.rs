use crate::models::{SimilarOutput, Similar, VerseOutput};
use crate::models::Database;
use crate::utils::sort;

pub fn get(dbs: &Database, kalima: &str) -> Vec<SimilarOutput> {
    let similar_db = &dbs.similar_db;

    let mut similars: Vec<SimilarOutput> = similar_db
        .iter()
        .filter_map(|result| {
            let (key, value) = result.ok()?;
            let key = std::str::from_utf8(&key).ok()?;
            if key == kalima {
                // Deserialize the value into Similar
                let similar: Similar = bincode::deserialize(&value).ok()?;

                // Convert Similar to SimilarOutput
                let similar_output = convert_to_output(dbs, &similar);
                Some(similar_output)
            } else {
                None
            }
        })
        .collect();

    sort::sort_similars(&mut similars);

    similars
}

fn convert_opposites(dbs: &Database, kalimas: &[String]) -> Option<Vec<VerseOutput>> {
    let mut verse_outputs = Vec::new();
    for kalima in kalimas {
        let result = dbs.similar_db.get(kalima);
        match result {
            Ok(Some(ivec)) => {
                // Deserialize the value into Similar
                let similar: Similar = bincode::deserialize(&ivec).unwrap();
                let verses = similar.verses.clone();
                verse_outputs.extend(verses);
            },
            Ok(None) => {
                // handle the case where there is no entry for the given key
                // this block could be left empty if there's no specific action needed
            },
            Err(e) => {
                // handle the error, you could print it or return it
                eprintln!("Database error: {}", e);
                // you could decide to return from the function here
            }
        }
    }
    if verse_outputs.is_empty() {
        None
    } else {
        Some(verse_outputs)
    }
}

pub fn convert_to_output(dbs: &Database, similar: &Similar) -> SimilarOutput {
    let verses = similar.verses.clone();
    let mut opposites = None;

    if let Some(opposite_similars) = &similar.opposites {
        if !opposite_similars.is_empty() {
            // info!("Found opposite similars: {:?}", opposite_similars);
            opposites = convert_opposites(dbs, opposite_similars);
        }
    }

    SimilarOutput {
        verses,
        opposites,
        kalima: similar.kalima.clone(),
    }
}