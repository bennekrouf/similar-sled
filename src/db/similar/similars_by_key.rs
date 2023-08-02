use crate::domain::coran::models::{SimilarOutput, Similar, Verse, VerseOutput};
use crate::models::Database;
use crate::utils::sort;
use crate::db::chapter::chapter_name;

pub fn get(dbs: &Database, similar_key: &str) -> Vec<SimilarOutput> {
    let similar_db = &dbs.similar_db;

    let mut similars: Vec<SimilarOutput> = similar_db
        .iter()
        .filter_map(|result| {
            let (key, value) = result.ok()?;
            let key = std::str::from_utf8(&key).ok()?;
            if key == similar_key {
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

fn get_sourate(dbs: &Database, verse: &Verse) -> String {
    // Some logic to get the sourate name from the verse chapter
    let chapter_name_result = chapter_name::get(dbs, verse.chapter as u8);
    match chapter_name_result {
        Ok(Some(name)) => name,
        Ok(None) | Err(_) => String::from("Default Chapter"),
    }
}

fn convert_to_output(dbs: &Database, similar: &Similar) -> SimilarOutput {
    let verses_output: Vec<VerseOutput> = similar.verses.iter().map(|verse| {
        // Conversion logic from Verse to VerseOutput
        VerseOutput {
            verse: verse.clone(),
            chapter: verse.chapter,
            sourate: get_sourate(dbs, verse),  // get the sourate string
        }
    }).collect();

    SimilarOutput {
        verses: verses_output,
        kalima: similar.kalima.clone(),
    }
}