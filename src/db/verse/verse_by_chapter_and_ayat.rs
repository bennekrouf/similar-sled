use crate::domain::coran::models::Database;

pub fn get(dbs: &Database, chapter: u32, ayah: u32) -> sled::Result<Option<String>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", chapter, ayah);
    let result = verse_db.get(&key)?;
    
    match result {
        Some(value) => {
            let verse_text = String::from_utf8_lossy(&value[..]).to_string();
            Ok(Some(verse_text))
        }
        None => Ok(None),
    }
}