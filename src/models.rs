use sled::Db;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub sourate: String,
    pub no: u8,
    pub mekka: bool,
    pub background_color: String,
    pub color: String,
    pub count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct VerseOutput {
    pub chapter_no: u32,
    pub verse_no: u32,
    pub sourate: Option<String>,
    pub ungrouped_text: Option<UngroupedText>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct YmlVerse {
    pub chapter_no: u32,
    pub verse_no: u32,
    pub text: String,
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Similar {
    pub kalima: String,
    pub opposites: Option<Vec<String>>,
    pub verses: Vec<VerseOutput>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct YmlSimilar {
    pub kalima: String,
    pub opposites: Option<Vec<String>>,
    pub verses: Vec<YmlVerse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutput {
    pub verses: Vec<VerseOutput>,
    pub opposites: Option<Vec<VerseOutput>>,
    pub kalima: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarOutputAdapted {
    pub verses: Vec<VerseOutput>,
    pub similars: Vec<VerseOutput>,
    pub opposites: Vec<VerseOutput>,
    pub kalima: String,
}

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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub struct Statement {
    pub kalima: String,
    pub verse: VerseOutput,
    pub has_opposites: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseOutput {
    pub kalima: String,
    pub verses: Vec<Statement>,
}
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Exercise {
    pub exercise_type: ExerciseType,
    pub statement: Statement,
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug)]
pub struct Database {
    pub chapter_db: Db,
    pub verse_db: Db,
    pub similar_db: Db,
    pub chapter_similar_db: Db,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            chapter_db: self.chapter_db.clone(),
            verse_db: self.verse_db.clone(),
            similar_db: self.similar_db.clone(),
            chapter_similar_db: self.chapter_similar_db.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub macos_path: String,
    pub debian_path: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Alternative {
    pub verse: Option<VerseOutput>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExerciseType {
    A,
    B,
    C,
}

impl ExerciseType {
    pub fn hide_fields(&self, exercise: &mut Exercise) {
        match self {
            ExerciseType::A => {
                exercise.statement.verse.ungrouped_text.as_mut().and_then(|text| text.discriminant.take());
            },
            ExerciseType::B => {
                exercise.statement.verse.sourate = None;
            },
            ExerciseType::C => {
                let text = &mut exercise.statement.verse.ungrouped_text;
                if let Some(ungrouped_text) = text {
                    // ungrouped_text.pre = None;
                    ungrouped_text.post = None;
                }
            }
        }
        
        // Applying the same rules to alternatives
        for alt in &mut exercise.alternatives {
            match self {
                ExerciseType::A => {
                    alt.verse.as_mut().map(|verse| verse.sourate = None);
                    alt.verse.as_mut().map(|verse| verse.chapter_no = 0);
                    alt.verse.as_mut().map(|verse| verse.verse_no = 0);
                    if let Some(verse) = &mut alt.verse {
                        verse.ungrouped_text.as_mut().map(|text| {
                            text.pre = None;
                            text.post = None;
                        });
                    }
                },
                ExerciseType::B => {
                    alt.verse.as_mut().and_then(|verse| verse.ungrouped_text.as_mut().and_then(|text| text.discriminant.take()));
                },
                ExerciseType::C => {
                    if let Some(verse) = &mut alt.verse {
                        verse.ungrouped_text.as_mut().map(|text| {
                            text.pre = None;
                        });
                    }
                }
            }
        }
    }
}

