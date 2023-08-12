use bincode;
use crate::{models::Database, models::Similar};

pub fn similars_insert(dbs: &Database, similar: &Similar) {
    let similar_db = &dbs.similar_db;
    
    // Serialize the whole Similar struct
    let serialized_similar = bincode::serialize(similar).unwrap();
    
    similar_db
        .insert(similar.kalima.clone(), serialized_similar)
        .expect("Failed to insert similar");
}