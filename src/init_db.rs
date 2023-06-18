use std::fs::File;
use std::io::Read;
use crate::models::{Chapter, Similar};
use crate::utils;
// use serde::Serialize;
use bincode;

fn load_chapters_from_yaml(file_path: &str) -> Vec<Chapter> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    serde_yaml::from_str(&contents).expect("Unable to parse YAML data")
}

fn load_similars_from_yaml(path: &str) -> Result<Vec<Similar>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let similars: Vec<Similar> = serde_yaml::from_str(&contents)?;
    Ok(similars)
}

pub fn init_chapters(db: &sled::Db) {
    if db.is_empty() {
        let chapters = load_chapters_from_yaml("data/chapters.yaml");
    
        for chapter in chapters {
            utils::insert_chapter(&db, &chapter).unwrap();
        }
    }
}

pub fn init_similars(similar_db: &sled::Db, verse_db: &sled::Db) {
    let similars_yaml = load_similars_from_yaml("data/similars.yaml").expect("Failed to load YAML file");

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
            utils::insert_verse(&verse_db, &verse).unwrap();
        }
    }
}