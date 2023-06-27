use bincode;
use crate::files::similars_from_yaml;
use crate::init::verse;
use crate::models::Similar;
use crate::utils::yml_path;

pub fn get(
    chapter: u32, 
    similar_db: &sled::Db, 
    verse_db: &sled::Db, 
    verse_similar_db: &sled::Db
) -> Vec<Similar> {
    let mut similars = Vec::new();

    // Iterate over all Verses in the chapter
    for ayat_ivec in verse_db.scan_prefix(format!("{}:", chapter)) {
        // The key is the IVec, the value is the Result<(IVec, IVec), sled::Error>
        let (verse_key, _value) = ayat_ivec.unwrap();

        // Fetch the list of Similar keys for this Verse
        let similar_keys: Vec<String> = verse_similar_db
            .get(&verse_key)
            .unwrap()
            .map(|ivec| bincode::deserialize(&ivec).unwrap())
            .unwrap_or_else(Vec::new);

        // Fetch each Similar using the keys and add it to the list
        for similar_key in similar_keys {
            let similar: Similar = similar_db
                .get(similar_key)
                .unwrap()
                .map(|ivec| bincode::deserialize(&ivec).unwrap())
                .unwrap();
            similars.push(similar);
        }
    }

    similars
}
