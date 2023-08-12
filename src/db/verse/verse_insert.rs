use sled::IVec;
use crate::models::Verse;
use crate::models::Database;

pub fn insert(dbs: &Database, verse: &Verse) -> sled::Result<Option<IVec>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", verse.chapter_no, verse.ayah);

    verse_db.insert(&key, verse.text.as_str())
}
