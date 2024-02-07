use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct YmlVerse {
    pub chapter_no: u32,
    pub verse_no: u32,
    pub text: String,
    pub previous: Option<String>,
    pub next: Option<String>,
}