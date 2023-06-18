use sled::{Db, IVec};
use bincode;
use serde::{Serialize, Deserialize};

use crate::models::Chapter;
use std::fs::File;
use std::io::Read;

pub fn load_chapters_from_yaml(file_path: &str) -> Vec<Chapter> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    serde_yaml::from_str(&contents).expect("Unable to parse YAML data")
}

pub fn get_chapter_name(db: &Db, chapter_no: u8) -> sled::Result<Option<String>> {
    let key = chapter_no.to_be_bytes().to_vec();
    match db.get(&key)? {
        Some(value) => {
            let chapter: Chapter = bincode::deserialize(&value).expect("Failed to deserialize");
            Ok(Some(chapter.name))
        },
        None => Ok(None),
    }
}

pub fn display(value: &Option<sled::IVec>) {
    if let Some(v) = value {
        if let Ok(s) = std::str::from_utf8(v) {
            println!("Value: {}", s);
        } else {
            println!("Invalid UTF-8 sequence");
        }
    } else {
        println!("No value found");
    }
}

pub fn count(db: &sled::Db) {
    let count = db.iter().keys().count();
    println!("There are {} keys in the database", count);
}

pub fn insert_chapter(db: &Db, chapter: &Chapter) -> sled::Result<Option<IVec>> {
    let key = chapter.no.to_be_bytes().to_vec();
    let value = bincode::serialize(chapter).expect("Failed to serialize");
    db.insert(key, value)
}