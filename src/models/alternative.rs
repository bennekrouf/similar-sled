use serde::{Serialize, Deserialize};
use super::verse_output::VerseOutput;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Alternative {
    pub verse: Option<VerseOutput>,
}