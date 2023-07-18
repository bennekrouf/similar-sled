use sled::IVec;
use crate::models::{Verse, Database};

pub fn insert(dbs: &Database, verse: &Verse) -> sled::Result<Option<IVec>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", verse.chapter, verse.ayat);

    verse_db.insert(&key, verse.text.as_str())
}
