use sled::{Db, IVec};
use crate::models::Verse;

pub fn insert(db: &Db, verse: &Verse) -> sled::Result<Option<IVec>> {
    let key = format!("{}-{}", verse.chapter, verse.ayat);
    db.insert(&key, verse.text.as_str())
}
