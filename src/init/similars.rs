use bincode;
use crate::init::similars_from_yaml;
use crate::init::verse;
use crate::models::Similar;
use crate::utils::yml_path;

pub fn init(similar_db: &sled::Db, verse_db: &sled::Db, verse_similar_db: &sled::Db) {
    let data_folder_path = yml_path::get_data_folder_path();

    let similars_yaml = similars_from_yaml::load(data_folder_path
        .join("similars.yaml")
        .to_str()
        .unwrap())
        .expect("Failed to load YAML file");

    for similar in similars_yaml {
        let kalima = similar.kalima.clone();

        let verse_references: Vec<(u32, u32)> = similar
            .verses
            .iter()
            .map(|verse| (verse.chapter, verse.ayat))
            .collect();

        let serialized_references = bincode::serialize(&verse_references).unwrap();
        similar_db
            .insert(kalima.clone(), serialized_references)
            .expect("Failed to insert similar");

        let verses = similar.verses;
        for verse in verses {
            verse::insert(&verse_db, &verse).unwrap();

            // Get the current list of Similar keys for this Verse, if it exists
            let verse_key = format!("{}:{}", verse.chapter, verse.ayat);
            let mut similar_keys: Vec<String> = verse_similar_db
                .get(&verse_key)
                .unwrap()
                .map(|ivec| bincode::deserialize(&ivec).unwrap())
                .unwrap_or_else(Vec::new);

            // Add the current Similar key to the list
            similar_keys.push(kalima.clone());

            // Store the updated list of Similar keys
            let serialized_keys = bincode::serialize(&similar_keys).unwrap();
            verse_similar_db
                .insert(verse_key, serialized_keys)
                .expect("Failed to insert verse-similar mapping");
        }
    }
}


pub fn get_similars_for_chapter(
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
