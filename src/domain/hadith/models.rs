use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub raoui: String,
    pub to: Option<Vec<Node>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Kitab {
    pub name: String,
    pub riwayate: Vec<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hadith {
    pub matan: String,
    pub koutoub: Vec<Kitab>,
    pub references: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mousned {
    pub ahadith: Vec<Hadith>,
    #[serde(skip)]
    pub sahib: Option<String>,
}