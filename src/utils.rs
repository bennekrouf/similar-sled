use sled::{Db, IVec};
use bincode;
// use serde_json;
use crate::models::{Chapter, Verse};
// use std::borrow::Cow;

pub fn count(db: &sled::Db) {
    let count = db.iter().keys().count();
    println!("There are {} keys in the database", count);
}

pub fn insert_chapter(db: &Db, chapter: &Chapter) -> sled::Result<Option<IVec>> {
    let key = chapter.no.to_be_bytes().to_vec();
    let value = bincode::serialize(chapter).expect("Failed to serialize");
    db.insert(key, value)
}

pub fn insert_verse(db: &Db, verse: &Verse) -> sled::Result<Option<IVec>> {
    let key = format!("{}-{}", verse.chapter, verse.ayat);
    db.insert(&key, verse.text.as_str())
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

pub fn get_verses_by_chapter(db: &Db, chapter: u8) -> sled::Result<Vec<(String, String)>> {
    let prefix = format!("{}-", chapter);
    let mut verses = Vec::new();

    for result in db.scan_prefix(prefix) {
        if let Ok((key, value)) = result {
            let verse_key = String::from_utf8_lossy(&key).into_owned();
            let verse_text = String::from_utf8_lossy(&value).into_owned();
            verses.push((verse_key, verse_text));
        }
    }

    Ok(verses)
}