use rocket::serde::{Deserialize, Serialize};
use sled::Db;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Chapter {
    pub name: String,
    pub no: u8,
    pub mekka: bool,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    pub color: String,
    pub count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Verse {
    pub chapter_no: u32,
    pub ayah: u32,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct VerseOutput {
    pub chapter_no: u32,
    pub sourate: String,
    pub verse: Verse,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Similar {
    pub kalima: String,
    pub opposite_similars: Option<Vec<String>>,
    pub verses: Vec<Verse>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SimilarOutput {
    pub verses: Vec<VerseOutput>,
    pub opposites: Option<Vec<VerseOutput>>,
    pub kalima: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SimilarOutputAdapted {
    pub verses: Vec<VerseOutput>,
    pub similars: Vec<VerseOutput>,
    pub opposites: Vec<VerseOutput>,
    pub kalima: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct VerseUngrouped {
    pub kalima: String,
    pub chapter_no: u32,
    pub chapter_name: String,
    pub ayah: u32,
    pub pre: Option<String>,
    pub discriminant: Option<String>,
    pub post: Option<String>,
    pub has_opposites: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ExerciseOutput {
    pub kalima: String,
    pub verses: Vec<VerseUngrouped>,
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
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub macos_path: String,
    pub debian_path: String,
    pub port: u16,
}

