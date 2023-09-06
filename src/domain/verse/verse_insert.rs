use crate::models::{Database, VerseOutput};
use sled::{self, IVec};
use bincode;

pub fn insert(dbs: &Database, verse: &VerseOutput) -> sled::Result<Option<IVec>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", verse.chapter_no, verse.verse_no);

    // let log = &verse;
    // print!("LOG {:?}", log);
    // Serialize UngroupedText using bincode
    let bincode_bytes = bincode::serialize(&verse.ungrouped_text).unwrap();

    // Use the ? operator to handle potential errors from insert
    // Dereference bincode_bytes so it becomes &[u8]
    verse_db.insert(&key, &*bincode_bytes)?;

    Ok(None)
}
