use sled::{Db, IVec};
use crate::models::Chapter;
use bincode;

pub fn insert(db: &Db, chapter: &Chapter) -> sled::Result<Option<IVec>> {
    let key = chapter.no.to_be_bytes().to_vec();
    let value = bincode::serialize(chapter).expect("Failed to serialize");
    db.insert(key, value)
}
