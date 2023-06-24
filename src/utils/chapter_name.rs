use sled::Db;
use bincode;
use crate::models::Chapter;

pub fn get(db: &Db, chapter_no: u8) -> sled::Result<Option<String>> {
    let key = chapter_no.to_be_bytes().to_vec();
    match db.get(&key)? {
        Some(value) => {
            let chapter: Chapter = bincode::deserialize(&value).expect("Failed to deserialize");
            Ok(Some(chapter.name))
        },
        None => Ok(None),
    }
}