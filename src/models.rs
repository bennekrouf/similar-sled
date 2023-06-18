use serde::{Serialize, Deserialize};

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