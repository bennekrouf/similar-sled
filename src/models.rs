use sled::Db;

#[derive(Debug)]
pub struct Database {
    pub chapter_db: Db,
    pub verse_db: Db,
    pub similar_db: Db,
    pub verse_similar_db: Db,
    pub mousned_db: Db,
    pub abwab_db: Db,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            chapter_db: self.chapter_db.clone(),
            verse_db: self.verse_db.clone(),
            similar_db: self.similar_db.clone(),
            verse_similar_db: self.verse_similar_db.clone(),
            // Hadith
            mousned_db: self.mousned_db.clone(),
            abwab_db: self.abwab_db.clone(),
        }
    }
}