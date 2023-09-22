use crate::models::Database;

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