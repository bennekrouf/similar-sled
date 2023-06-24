use crate::init::chapters_from_yaml;
use crate::init::chapter;

pub fn init(db: &sled::Db) {
    if db.is_empty() {
        let chapters = chapters_from_yaml::load("data/chapters.yaml");
    
        for chapter in chapters {
            chapter::insert(&db, &chapter).unwrap();
        }
    }
}