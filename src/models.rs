use sled::Db;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub sourate: String,
    pub no: u8,
    pub mekka: bool,
    pub backgroundColor: String,
    pub color: String,
    pub count: Option<u32>,
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub struct Verse {
//     pub chapter_no: u32,
//     pub ayah: u32,
//     pub text: String,
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct VerseOutput {
    pub chapter_no: u32,
    pub ayah: u32,
    pub sourate: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Similar {
    pub kalima: String,
    pub opposite_similars: Option<Vec<String>>,
    pub verses: Vec<VerseOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutput {
    pub verses: Vec<VerseOutput>,
    pub opposites: Option<Vec<VerseOutput>>,
    pub kalima: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutputAdapted {
    pub verses: Vec<VerseOutput>,
    pub similars: Vec<VerseOutput>,
    pub opposites: Vec<VerseOutput>,
    pub kalima: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub struct Statement {
    pub kalima: String,
    pub verse: VerseOutput,
    pub pre: Option<String>,
    pub discriminant: Option<String>,
    pub post: Option<String>,
    pub has_opposites: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseOutput {
    pub kalima: String,
    pub verses: Vec<Statement>,
}
#[derive(Debug)]
pub struct Database {
    pub chapter_db: Db,
    pub verse_db: Db,
    pub similar_db: Db,
    pub chapter_similar_db: Db,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            chapter_db: self.chapter_db.clone(),
            verse_db: self.verse_db.clone(),
            similar_db: self.similar_db.clone(),
            chapter_similar_db: self.chapter_similar_db.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub macos_path: String,
    pub debian_path: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExerciseType {
    A,
    B,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Alternative {
    pub content: String,
    pub ayah: Option<VerseOutput>,
}