use crate::models::{SimilarOutput, Database};
use super::similars_by_key;

pub fn get_chapter_similars(dbs: &Database, chapter: u32) -> Vec<SimilarOutput> {
    let chapter_key = chapter.to_string();
    let similar_keys = get_similar_keys(dbs, &chapter_key);
    let mut similar_objects: Vec<SimilarOutput> = Vec::new();

    for similar_key in similar_keys {
        let similar = similars_by_key::get_similars_db_by_key(dbs, &similar_key);
        similar_objects.extend(similar);
    }
    
    similar_objects
}


fn get_similar_keys(dbs: &Database, key: &str) -> Vec<String> {
    let serialized_keys = dbs
        .verse_similar_db
        .get(key.as_bytes())
        .unwrap_or(None)
        .unwrap_or_default();

    bincode::deserialize(&serialized_keys)
        .unwrap_or_default()
}