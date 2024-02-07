use serde::{Serialize, Deserialize};
use super::verse_output::VerseOutput;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Similar {
    pub kalima: String,
    pub opposites: Option<Vec<String>>,
    pub verses: Vec<VerseOutput>,
}
