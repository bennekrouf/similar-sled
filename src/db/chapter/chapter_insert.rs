use sled::IVec;
use crate::models::Chapter;
use crate::models::Database;
use bincode;

pub fn insert(dbs: &Database, chapter: &Chapter) -> sled::Result<Option<IVec>> {
    let chapter_db = &dbs.chapter_db;

    let key = chapter.no.to_be_bytes().to_vec();
    // println!("Inserting chapter: {:?}", chapter);
    let value = bincode::serialize(chapter).expect("Failed to serialize");
    chapter_db.insert(key, value)
}
