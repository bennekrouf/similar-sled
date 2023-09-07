use rand::seq::SliceRandom;

use crate::domain::exercise::get_solution::get_solution;
use crate::domain::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::domain::exercise::select_random_verse_index::select_random_verse_index;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::models::{ExerciseType, Database, Alternative, Exercise};
use crate::utils::deduplicate_by_field::deduplicate_by_field;

pub fn generate(dbs: &Database, kalima: String, exercise_type: ExerciseType) -> Option<Exercise> {
    let mut solutions = get_solution(dbs, &kalima);
    if solutions.is_empty() { return None; }
    
    let exercise = &mut solutions[0];
    let valid_verse_index = select_random_verse_index(&exercise.verses);
    // let log = exercise.verses.get_mut(valid_verse_index);

    // println!("exercise.verses.get_mut(valid_verse_index) {:?}", log);
    if let Some(ref mut valid_verse) = exercise.verses.get_mut(valid_verse_index) {
        valid_verse.verse.sourate = Some(sourate_name_from_verse(dbs, &valid_verse.verse));
    }

    let exclude_value = Some(Alternative { 
        verse: Some(exercise.verses[valid_verse_index].verse.clone())
    });

    // Convert discriminants (assuming they are chapter names) to the Alternative format
    let mut alternatives = extract_and_shuffle_options(&mut exercise.verses, 
    |statement| {
        let alternative = Alternative { 
            verse: Some(statement.verse.clone())
        };
        
        Some(alternative)
    },
    &exclude_value);

    for alternative in &mut alternatives {
        if let Some(ref mut verse) = alternative.verse {
            verse.sourate = Some(sourate_name_from_verse(dbs, verse));
        }
    }

    alternatives = deduplicate_by_field(alternatives.clone(), |alt| {
        alt.verse
            .as_ref()
            .and_then(|verse| {
                verse.ungrouped_text.as_ref().map(|text| text.discriminant.clone())
            })
    });
    alternatives = deduplicate_by_field(alternatives.clone(), |alt| alt.verse.as_ref().unwrap().sourate.clone());
    alternatives = deduplicate_by_field(alternatives.clone(), |alt| Some(alt.verse.as_ref().unwrap().verse_no));
    alternatives.shuffle(&mut rand::thread_rng());

    // Limit to 3 possible answers (excluding the correct answer which we will add later)
    alternatives.truncate(2);

    let valid_verse = exercise.verses.get_mut(valid_verse_index).unwrap();

    alternatives.push(Alternative { verse: Some(valid_verse.verse.clone()) });
    alternatives.shuffle(&mut rand::thread_rng());
    
    let mut generated_exercise = Some(Exercise {
        statement: valid_verse.clone(),
        alternatives, // Pass the cloned alternatives here
        exercise_type: ExerciseType::A
    });

    if let Some(ref mut exercise) = generated_exercise {
        exercise_type.hide_fields(exercise);
    }
    
    generated_exercise

}
