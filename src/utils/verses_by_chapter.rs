use sled::Db;

pub fn get(db: &Db, chapter: u8) -> sled::Result<Vec<(String, String)>> {
    let prefix = format!("{}-", chapter);
    let mut verses = Vec::new();

    for result in db.scan_prefix(prefix) {
        if let Ok((key, value)) = result {
            let verse_key = String::from_utf8_lossy(&key).into_owned();
            let verse_text = String::from_utf8_lossy(&value).into_owned();
            verses.push((verse_key, verse_text));
        }
    }

    Ok(verses)
}