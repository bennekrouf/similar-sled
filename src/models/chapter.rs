use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub sourate: String,
    pub no: u8,
    pub mekka: bool,
    pub background_color: String,
    pub color: String,
    pub count: Option<u32>,
    pub count_ayat: Option<u32>,
}