use bincode;
use std::collections::HashSet;
use crate::files::similars_from_yaml::load;
use crate::domain::verse::verse_insert;
use crate::domain::similar::similars_insert;
use crate::models::{VerseOutput, Database};

pub fn init(dbs: &Database) {
    let similars = load().expect("Failed to load YAML file");

    for similar in similars {
        similars_insert::similars_insert(&dbs, &similar);
        let kalima = similar.kalima.clone();

        for verse in similar.verses {
            verse_insert::insert(&dbs, &verse).unwrap();
            update_verse_similar_mapping(&dbs, &verse, &kalima);
        }
    }
}

fn update_verse_similar_mapping(dbs: &Database, verse: &VerseOutput, kalima: &str) {
    let chapter_similar_db = &dbs.chapter_similar_db;
    let chapter_no = verse.chapter_no.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_no);
    let mut similar_keys_set: HashSet<String> = similar_keys.into_iter().collect();

    if !similar_keys_set.contains(kalima) {
        similar_keys_set.insert(kalima.to_string());
    }

    let similar_keys: Vec<String> = similar_keys_set.into_iter().collect();

    let serialized_similar_keys = bincode::serialize(&similar_keys).unwrap();
    chapter_similar_db
        .insert(chapter_no, serialized_similar_keys)
        .expect("Failed to insert verse-similar mapping");
}

fn get_similar_keys(dbs: &Database, chapter_no: &str) -> Vec<String> {
    let chapter_similar_db = &dbs.chapter_similar_db;
    chapter_similar_db
        .get(chapter_no)
        .unwrap()
        .map(|ivec| bincode::deserialize(&ivec).unwrap())
        .unwrap_or_else(Vec::new)
}