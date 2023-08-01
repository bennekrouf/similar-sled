use crate::domain::coran::models::SimilarOutput;
use crate::models::Database;
use crate::db::similar::similar_output_format;
use crate::utils::sort;
use sled::IVec;

// Extracted helper function to get the key from the result
fn get_key_from_result(result: &Result<(IVec, IVec), sled::Error>) -> Result<String, sled::Error> {
    result.clone().map(|(key, _)| String::from_utf8_lossy(&key).into_owned())
}

// Extracted helper function to get the value from the result
fn get_value_from_result(result: &Result<(IVec, IVec), sled::Error>, similar_key: &str, dbs: &Database) -> Result<SimilarOutput, sled::Error> {
    result.clone().map(|(_, similar_value)| {
        let verses: Vec<String> = bincode::deserialize(&similar_value).expect("Failed to deserialize references");
        similar_output_format::create_similar_output(similar_key.to_owned(), &verses, dbs)
    })
}

pub fn get_similars_db_by_key(dbs: &Database, similar_key: &str) -> Vec<SimilarOutput> {
    let similar_db = &dbs.similar_db;

    let mut similars: Vec<SimilarOutput> = similar_db
        .iter()
        .filter_map(|result| match get_key_from_result(&result) {
            Ok(key) => if key == similar_key {
                Some(result)
            } else {
                None
            },
            Err(_) => None,
        })
        .filter_map(|result| get_value_from_result(&result, similar_key, dbs).ok())
        .collect();

    sort::sort_similars(&mut similars);

    similars
}
