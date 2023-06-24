use bincode;
use crate::init::similars_from_yaml;
use crate::init::verse;
use crate::utils::yml_path;

pub fn init(similar_db: &sled::Db, verse_db: &sled::Db) {
    let data_folder_path = yml_path::get_data_folder_path();

    let similars_yaml = similars_from_yaml::load(data_folder_path
        .join("similars.yaml")
        .to_str()
        .unwrap())
        .expect("Failed to load YAML file");
    // let similars_yaml = similars_from_yaml::load(data_folder_path.join("similars.yaml").to_str().unwrap());
    for similar in similars_yaml {

        let verse_references: Vec<(u32, u32)> = similar
            .verses
            .iter()
            .map(|verse| (verse.chapter, verse.ayat))
            .collect();

        let serialized_references = bincode::serialize(&verse_references).unwrap();
        similar_db
            .insert(similar.text, serialized_references)
            .expect("Failed to insert similar");


        let verses = similar.verses;
        for verse in verses {
            verse::insert(&verse_db, &verse).unwrap();
        }
    }
}