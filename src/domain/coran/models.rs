use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub name: String,
    pub no: u8,
    pub mekka: bool,
    pub backgroundColor: String,
    pub color: String,
    pub count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Verse {
    pub text: String,
    pub ayah: u32,
    pub chapter: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Similar {
    pub kalima: String,
    pub opposite_similars: Option<Vec<String>>,
    pub verses: Vec<Verse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerseOutput {
    pub verse: Verse,
    pub chapter: u32,
    pub sourate: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutput {
    pub verses: Vec<VerseOutput>,
    // pub opposites: Option<Vec<VerseOutput>>,
    pub kalima: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutputAdapted {
    pub verses: Vec<VerseOutput>,
    pub similars: Vec<VerseOutput>,
    // pub opposites: Vec<VerseOutput>,
    pub kalima: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VerseSimilarOutput {
    pub verse: Verse,
    pub similar_keys: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChapterSimilarOutput {
    pub chapter_name: String,
    pub similar_objects: Vec<Similar>,
}