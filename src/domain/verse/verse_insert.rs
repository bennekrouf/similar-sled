use sled::IVec;
use crate::models::VerseOutput;
use crate::models::Database;

pub fn insert(dbs: &Database, verse: &VerseOutput) -> sled::Result<Option<IVec>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", verse.chapter_no, verse.ayah);

    // Remove the "غغغ" pattern from the text
    let cleaned_text = verse.text.replace("غغغ", "");

    verse_db.insert(&key, cleaned_text.as_str())
}
