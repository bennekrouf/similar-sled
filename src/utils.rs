use sled::Db;
use bincode;
// use serde_json;
use crate::models::Chapter;
// use std::borrow::Cow;

use std::path::{Path, PathBuf};
use users::get_current_username;
use dirs::home_dir;

pub fn get_data_folder_path() -> PathBuf {
    let username = get_current_username()
        .expect("Failed to retrieve current username")
        .into_string()
        .expect("Failed to convert username to string");

    if let Some(mut home_path) = home_dir() {
        home_path.push("dbs");
        return home_path;
    }

    Path::new("/tmp").join(&username).join("dbs")
}

pub fn count(db: &sled::Db) {
    let count = db.iter().keys().count();
    println!("There are {} keys in the database", count);
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

pub fn get_verse_by_chapter_and_ayat(db: &Db, chapter: u32, ayat: u32) -> sled::Result<Option<String>> {
    let key = format!("{}-{}", chapter, ayat);
    let result = db.get(&key)?;
    
    match result {
        Some(value) => {
            let verse_text = String::from_utf8_lossy(&value[..]).to_string();
            Ok(Some(verse_text))
        }
        None => Ok(None),
    }
}