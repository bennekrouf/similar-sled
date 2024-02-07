use serde::{Serialize, Deserialize};
use super::verse_output::VerseOutput;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub struct Statement {
    pub kalima: String,
    pub verse: VerseOutput,
    pub has_opposites: bool,
}
