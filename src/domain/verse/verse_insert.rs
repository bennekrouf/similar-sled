use crate::models::{Database, VerseOutput};
use sled::{self, IVec};
use bincode;

pub fn insert(dbs: &Database, verse: &VerseOutput) -> sled::Result<Option<IVec>> {
    let verse_db = &dbs.verse_db;

    let key = format!("{}:{}", verse.chapter_no, verse.verse_no);

    // let log = &verse;
    // print!("LOG {:?}", log);
    let bincode_bytes = bincode::serialize(&verse.ungrouped_text).unwrap();

    // Use the ? operator to handle potential errors from insert
    // Dereference bincode_bytes so it becomes &[u8]
    verse_db.insert(&key, &*bincode_bytes)?;

    Ok(None)
}

pub fn count_verses_in_chapter(dbs: &Database, chapter_no: u32) -> sled::Result<u32> {
    let verse_db = &dbs.verse_db;

    // Create a scan range for the chapter.
    let start_key = format!("{}:", chapter_no);  // Start key
    let end_key = format!("{}:", chapter_no + 1); // Exclusive end key

    // Initialize a counter for the verses.
    let mut verse_count = 0;

    // Scan through the database, only considering keys that start with "{chapter_no}:"
    for item in verse_db.range(start_key..end_key) {
        match item {
            Ok((key, _)) => {
                // Convert key to string for comparison
                let key_str = String::from_utf8_lossy(&key);
                if key_str.starts_with(&format!("{}:", chapter_no)) {
                    verse_count += 1;
                }
            },
            Err(_) => {
                // Handle error as you see fit.
            }
        }
    }

    Ok(verse_count)
}

pub fn debug_print_keys(dbs: &Database, chapter_no: u32) {
    let verse_db = &dbs.verse_db;
    let start_key = format!("{}:", chapter_no); // start key for the chapter
    let end_key = format!("{}:", chapter_no + 1); // end key for the chapter
    
    // Iterate over the keys in the range and print them
    for key in verse_db.range(start_key..end_key) {
        match key {
            Ok((k, _)) => {
                let k_str = String::from_utf8_lossy(&k);
                println!("Key: {}", k_str);
            },
            Err(e) => {
                println!("Error while iterating: {:?}", e);
            }
        }
    }
}
