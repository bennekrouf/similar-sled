use bincode;
use crate::models::Database;

pub fn similars_insert(dbs: &Database, kalima: &str, verse_references: &[String]) {
    let similar_db = &dbs.similar_db;

    // println!("insert_similar kalima {:?} verse_references {:?}", kalima, &verse_references);
    
    let serialized_references = bincode::serialize(verse_references).unwrap();
    similar_db
        .insert(kalima.to_string(), serialized_references)
        .expect("Failed to insert similar");
}