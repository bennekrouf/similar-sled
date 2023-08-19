use crate::models::{Database, Similar};

pub fn get_count_for_kalima(dbs: &Database, kalima: &str) -> usize {
    let similar_db = &dbs.similar_db;

    // Use the `find_map` iterator method to find the first (and likely only) 
    // entry with the given kalima, and then return the count of its verses.
    similar_db
        .iter()
        .find_map(|result| {
            let (key, value) = result.ok()?;
            let key = std::str::from_utf8(&key).ok()?;
            if key == kalima {
                // Deserialize the value into Similar
                let similar: Similar = bincode::deserialize(&value).ok()?;
                Some(similar.verses.len())
            } else {
                None
            }
        })
        .unwrap_or(0)  // Return 0 if no match is found
}