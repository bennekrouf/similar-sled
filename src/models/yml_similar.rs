use serde::{Serialize, Deserialize};
use super::yml_verse::YmlVerse;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct YmlSimilar {
    pub kalima: String,
    pub opposites: Option<Vec<String>>,
    pub verses: Vec<YmlVerse>,
}