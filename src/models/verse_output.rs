use serde::{Serialize, Deserialize};

use super::ungrouped_text::UngroupedText;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct VerseOutput {
    pub chapter_no: u32,
    pub verse_no: u32,
    pub sourate: Option<String>,
    pub ungrouped_text: Option<UngroupedText>,
}