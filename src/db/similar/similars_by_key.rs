use crate::models::{SimilarOutput, Database};
use crate::db::similar::similar_output_format;
use crate::utils::sort;

pub fn get_similars_db_by_key(dbs: &Database, similar_key: &str) -> Vec<SimilarOutput> {
    let similar_db = &dbs.similar_db;
    let mut similars = similar_db
        .iter()
        .filter(|result| {
            let (key, _) = result.as_ref().expect("Failed to retrieve similar");
            String::from_utf8_lossy(&key).eq(similar_key)
        })
        .map(|result| {
            let (_, similar_value) = result.expect("Failed to retrieve similar");
            let kalima = similar_key.to_owned();
            let verses: Vec<String> =
                bincode::deserialize(&similar_value).expect("Failed to deserialize references");
            similar_output_format::create_similar_output(kalima, &verses, dbs)
        })
        .collect::<Vec<SimilarOutput>>();

    sort::sort_similars(&mut similars);
    similars
}