use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::models::{Statement, ExerciseType, VerseOutput};

pub fn extract_and_shuffle_options(
    statements: &mut [Statement],
    exercise_type: ExerciseType,
    exclude_verse: &Option<VerseOutput>,
) -> Vec<String> {
    let mut seen = HashSet::new();

    let mut options: Vec<String> = statements.iter_mut()
        .filter_map(|statement| {
            let option = match exercise_type {
                ExerciseType::FindDscriminant => {
                    statement.verse.ungrouped_text.as_ref().and_then(|ut| ut.discriminant.clone())
                },
                ExerciseType::FindSourate => {
                    statement.verse.sourate.clone()
                },
                ExerciseType::C => {
                    // handle other extraction logic here
                    None // placeholder
                },
            };
            
            let exclude_value = match exercise_type {
                ExerciseType::FindDscriminant => exclude_verse.as_ref().and_then(|verse| verse.ungrouped_text.as_ref().and_then(|ut| ut.discriminant.clone())),
                ExerciseType::FindSourate => exclude_verse.as_ref().and_then(|verse| verse.sourate.clone()),
                ExerciseType::C => {
                    // handle other extraction logic here
                    None // placeholder
                },
            };
            
            if option.is_some() && seen.insert(option.as_ref().unwrap().clone()) {
                if let Some(exclude_str) = &exclude_value {
                    if option.as_ref() == Some(exclude_str) {
                        return None;
                    }
                }
                option
            } else {
                None
            }
        })
        .collect();

    options.shuffle(&mut rand::thread_rng());
    options
}
