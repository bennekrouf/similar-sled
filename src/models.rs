use serde::{Serialize, Deserialize};
use sled::Db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub name: String,
    pub no: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Verse {
    pub text: String,
    pub ayat: u32,
    pub chapter: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Similar {
    pub text: String,
    pub verses: Vec<Verse>,
}

pub struct Database {
    pub chapter_db: Db,
    pub verse_db: Db,
    pub similar_db: Db,
}