use std::fs::File;
use std::io::Read;

use crate::models::Chapter;
use crate::utils;

pub fn load_chapters_from_yaml(file_path: &str) -> Vec<Chapter> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    serde_yaml::from_str(&contents).expect("Unable to parse YAML data")
}

pub fn init_chapters(db: &sled::Db) {
    if db.is_empty() {
        let chapters = load_chapters_from_yaml("chapters.yaml");
    
        for chapter in chapters {
            utils::insert_chapter(&db, &chapter).unwrap();
        }
    }
}