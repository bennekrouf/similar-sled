use crate::models::VerseOutput;
use crate::models::Database;
use sled::{self, IVec};
use std::borrow::Cow;

pub fn insert(dbs: &Database, verse: &VerseOutput) -> sled::Result<Option<IVec>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", verse.chapter_no, verse.verse_no);

    // Clone the text to modify it
    let modified_text = verse.text.clone();

    // Convert the Option<String> to Cow<str> for efficient borrowing
    let text: Cow<str> = match modified_text {
        Some(ref t) => Cow::Borrowed(t),
        None => Cow::Borrowed(""),
    };

    // Remove the "غغغ" pattern from the text
    let cleaned_text = text.replace("غغغ", "");

    // Convert the modified text to bytes
    let text_bytes: &[u8] = cleaned_text.as_bytes();

    // Use the ? operator to handle potential errors from insert
    verse_db.insert(&key, text_bytes)?;

    Ok(None)
}