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
    pub chapter_no: u32,
    pub ayah: u32,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerseOutput {
    pub chapter_no: u32,
    pub sourate: String,
    pub verse: Verse,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Similar {
    pub kalima: String,
    pub opposite_similars: Option<Vec<String>>,
    pub verses: Vec<Verse>,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub struct ExerciseOutput {
    pub kalima: String,
    pub verses: Vec<VerseUngrouped>,
}