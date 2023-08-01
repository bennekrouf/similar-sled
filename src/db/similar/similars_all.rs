use crate::domain::coran::models::SimilarOutput;
use crate::models::Database;
use crate::db::similar::similar_output_format;
use crate::utils::sort;

pub fn get(dbs: &Database) -> Vec<SimilarOutput> {
    let similar_db = &dbs.similar_db;
    let mut similars = similar_db
        .iter()
        .map(|result| {
            let (similar_key, similar_value) = result.expect("Failed to retrieve similar");
            let kalima = String::from_utf8_lossy(&similar_key).into_owned();
            let verses: Vec<String> =
                bincode::deserialize(&similar_value).expect("Failed to deserialize references");
            similar_output_format::create_similar_output(kalima, &verses, dbs)
        })
        .collect::<Vec<SimilarOutput>>();

    sort::sort_similars(&mut similars);
    similars
}