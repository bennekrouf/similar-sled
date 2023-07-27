use bincode;
use std::collections::HashSet;
use crate::files::similars_from_yaml::load;
use crate::db::verse::verse_insert;
use crate::domain::coran::models::{Similar, Verse};
use crate::models::Database;
use crate::db::similar::similars_insert;

pub fn init(dbs: &Database) {
    let similars = load().expect("Failed to load YAML file");
    // process_similars(&dbs, similars);
    for similar in similars {
        let kalima = similar.kalima.clone();
        let verse_references = get_verse_references(&similar);

        similars_insert::similars_insert(&dbs, &kalima, &verse_references);

        for verse in similar.verses {
            verse_insert::insert(&dbs, &verse).unwrap();
            update_verse_similar_mapping(&dbs, &verse, &kalima);
        }
    }
}


fn get_verse_references(similar: &Similar) -> Vec<String> {
    similar
        .verses
        .iter()
        .map(|verse| format!("{}:{}", verse.chapter, verse.ayah))
        .collect()
}

fn update_verse_similar_mapping(dbs: &Database, verse: &Verse, kalima: &str) {
    let verse_similar_db = &dbs.verse_similar_db;
    let verse_key = verse.chapter.to_string();
    let similar_keys = get_similar_keys(dbs, &verse_key);
    let mut similar_keys_set: HashSet<String> = similar_keys.into_iter().collect();

    if !similar_keys_set.contains(kalima) {
        similar_keys_set.insert(kalima.to_string());
    }

    let similar_keys: Vec<String> = similar_keys_set.into_iter().collect();

    let serialized_keys = bincode::serialize(&similar_keys).unwrap();
    verse_similar_db
        .insert(verse_key, serialized_keys)
        .expect("Failed to insert verse-similar mapping");
}

fn get_similar_keys(dbs: &Database, verse_key: &str) -> Vec<String> {
    let verse_similar_db = &dbs.verse_similar_db;
    verse_similar_db
        .get(verse_key)
        .unwrap()
        .map(|ivec| bincode::deserialize(&ivec).unwrap())
        .unwrap_or_else(Vec::new)
}