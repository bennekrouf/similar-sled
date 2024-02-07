use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct UngroupedText {
    pub pre: Option<String>,
    pub discriminant: Option<String>,
    pub post: Option<String>,
}

impl Default for UngroupedText {
    fn default() -> Self {
        UngroupedText {
            pre: None,
            discriminant: None,
            post: None,
        }
    }
}
