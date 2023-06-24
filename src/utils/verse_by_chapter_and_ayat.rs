use sled::Db;

pub fn get(db: &Db, chapter: u32, ayat: u32) -> sled::Result<Option<String>> {
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