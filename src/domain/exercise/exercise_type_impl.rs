use crate::models::{ExerciseType, Exercise};

impl ExerciseType {
    pub fn hide_fields(&self, exercise: &mut Exercise) {
        match self {
            ExerciseType::FindDiscriminant => {
                exercise.statement.verse.ungrouped_text.as_mut().and_then(|text| text.discriminant.take());
            },
            ExerciseType::FindSourate => {
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
                ExerciseType::FindDiscriminant => {
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
                ExerciseType::FindSourate => {
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