use serde::{Serialize, Deserialize};
use super::verse_output::VerseOutput;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct SimilarOutputAdapted {
    pub verses: Vec<VerseOutput>,
    pub similars: Vec<VerseOutput>,
    pub opposites: Vec<VerseOutput>,
    pub kalima: String,
}