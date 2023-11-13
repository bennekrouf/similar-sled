use rand::seq::SliceRandom;
use std::collections::HashSet;
use crate::models::{Statement, ExerciseType, VerseOutput};

pub fn extract_and_shuffle_options(
    statements: &[Statement],
    exercise_type: ExerciseType,
    exclude_verse: &Option<VerseOutput>,
) -> Vec<(String, VerseOutput)> {
    let mut seen = HashSet::new();

    let exclude_value = get_exclude_value(exercise_type, exclude_verse);

    let mut results: Vec<(String, VerseOutput)> = statements.iter()
        .filter_map(|statement| {
            let option = get_option_value(exercise_type, statement);
            if option.as_ref() == exclude_value.as_ref() {
                return None;
            }

            option.and_then(|opt| {
                if seen.insert((opt.clone(), statement.verse.clone())) {
                    Some((opt, statement.verse.clone()))
                } else {
                    None
                }
            })
        })
        .collect();

    results.shuffle(&mut rand::thread_rng());
    results
}

fn get_option_value(exercise_type: ExerciseType, statement: &Statement) -> Option<String> {
    match exercise_type {
        ExerciseType::FindDiscriminant => statement.verse.ungrouped_text.as_ref().and_then(|ut| ut.discriminant.clone()),
        ExerciseType::FindSourate => statement.verse.sourate.clone(),
        ExerciseType::C => None, // Handle other cases or use a default
    }
}

fn get_exclude_value(exercise_type: ExerciseType, exclude_verse: &Option<VerseOutput>) -> Option<String> {
    match exercise_type {
        ExerciseType::FindDiscriminant => exclude_verse.as_ref().and_then(|verse| verse.ungrouped_text.as_ref().and_then(|ut| ut.discriminant.clone())),
        ExerciseType::FindSourate => exclude_verse.as_ref().and_then(|verse| verse.sourate.clone()),
        ExerciseType::C => None, // Handle other cases or use a default
    }
}
