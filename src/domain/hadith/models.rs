use serde::{Deserialize, Serialize};
use sled::Db;

#[derive(Debug, Clone)]
pub struct Database {
    pub mousned_db: Db,
    pub abwab_db: Db,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Mousned {
    pub sahib: String,
    pub rouate: Vec<Bab>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Bab {
    pub raoui: String,
    pub ahadith: Vec<Hadith>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Hadith {
    pub matan: String,
    pub riwayate: Vec<Riwaya>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Riwaya {
    pub kitab: String,
    pub references: Vec<String>,
    pub houkm: Option<Vec<String>>,
    pub asanid: Vec<Sanad>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Sanad {
    pub sanad: Either<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}