use std::{fs::File, path::PathBuf};
use std::io::Read;
use sled::{Db, IVec};

use crate::models::{Chapter, Similar, Database, Verse};
// use serde::Serialize;
use bincode;

pub fn insert_chapter(db: &Db, chapter: &Chapter) -> sled::Result<Option<IVec>> {
    let key = chapter.no.to_be_bytes().to_vec();
    let value = bincode::serialize(chapter).expect("Failed to serialize");
    db.insert(key, value)
}

pub fn insert_verse(db: &Db, verse: &Verse) -> sled::Result<Option<IVec>> {
    let key = format!("{}-{}", verse.chapter, verse.ayat);
    db.insert(&key, verse.text.as_str())
}

pub fn init_all_db(absolute_data_folder_path: &PathBuf) -> Database {
    // Construct the absolute paths to the database files
    let chapter_db_path = absolute_data_folder_path.join("chapter_db");
    let verse_db_path = absolute_data_folder_path.join("verse_db");
    let similar_db_path = absolute_data_folder_path.join("similar_db");

    print!("{:?}", chapter_db_path);

    // Open the Sled databases using the adjusted file paths
    let chapter_db = sled::open(chapter_db_path).expect("Failed to open chapter database");
    let verse_db = sled::open(verse_db_path).expect("Failed to open verse database");
    let similar_db = sled::open(similar_db_path).expect("Failed to open similar database");

    let database = Database {
        chapter_db,
        verse_db,
        similar_db,
    };

    init_chapters(&database.chapter_db);
    init_similars(&database.similar_db, &database.verse_db);
    database
}

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
            insert_chapter(&db, &chapter).unwrap();
        }
    }
}

// pub fn flush_db(db: &sled::Db) {
//     db.flush().unwrap();
// }

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
            insert_verse(&verse_db, &verse).unwrap();
        }
    }
}