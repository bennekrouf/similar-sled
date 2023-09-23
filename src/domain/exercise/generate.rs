use rand::seq::SliceRandom;

use crate::domain::exercise::get_solution::get_solution;
use crate::domain::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::domain::exercise::select_random_verse_index::select_random_verse_index;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::models::{ExerciseType, Database, Alternative, Exercise, VerseOutput, UngroupedText};

pub fn generate(dbs: &Database, kalima: String, exercise_type: ExerciseType) -> Option<Exercise> {
    let mut solutions = get_solution(dbs, &kalima);
    if solutions.is_empty() { return None; }
    
    let exercise = &mut solutions[0];
    let valid_verse_index = select_random_verse_index(&exercise.verses);
    // let log = exercise.verses.get_mut(valid_verse_index);

    if let Some(ref mut valid_verse) = exercise.verses.get_mut(valid_verse_index) {
        valid_verse.verse.sourate = Some(sourate_name_from_verse(dbs, &valid_verse.verse));
    }

    let exclude_verse = Some(exercise.verses[valid_verse_index].verse.clone());

    // Convert discriminants (assuming they are chapter names) to the Alternative format
    let extracted_values = extract_and_shuffle_options(&mut exercise.verses, exercise_type, &exclude_verse);
    let mut alternatives: Vec<Alternative> = extracted_values.into_iter().map(|value| {
        match exercise_type {
            ExerciseType::FindDiscriminant => {
                // Use discriminant to form alternative
                Alternative {
                    verse: Some(VerseOutput {
                        chapter_no: 0,
                        verse_no: 0,
                        sourate: None,
                        ungrouped_text: Some(UngroupedText {
                            discriminant: Some(value.0),
                            pre: None,
                            post: None,
                        }),
                    }),
                }
            },
            ExerciseType::FindSourate => {
                // Use sourate to form alternative
                Alternative {
                    verse: Some(VerseOutput {
                        sourate: Some(value.0),
                        chapter_no: value.1.chapter_no,
                        verse_no: value.1.verse_no,
                        ungrouped_text: None,
                    }),
                }
            },
            _ => unimplemented!(), // Handle other cases or use a default
        }
    }).collect();

    // Limit to 3 possible answers (excluding the correct answer which we will add later)
    alternatives.truncate(2);

    let valid_verse = exercise.verses.get_mut(valid_verse_index).unwrap();

    alternatives.push(Alternative { verse: Some(valid_verse.verse.clone()) });
    alternatives.shuffle(&mut rand::thread_rng());
    
    let mut generated_exercise = Some(Exercise {
        statement: valid_verse.clone(),
        alternatives, // Pass the cloned alternatives here
        exercise_type: exercise_type.clone(),
    });

    if let Some(ref mut exercise) = generated_exercise {
        exercise_type.hide_fields(exercise);
    }
    
    generated_exercise

}
