use rand::seq::SliceRandom;

use crate::domain::exercise::get_solution::get_solution;
use crate::domain::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::domain::exercise::select_random_verse_index::select_random_verse_index;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::models::{ExerciseType, Database, Alternative, Exercise};

use crate::utils::deduplicate_by_field::deduplicate_by_field;
pub fn generate(dbs: &Database, kalima: String) -> Option<Exercise> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }
    
    let exercise = &mut exercises[0];
    let selected_verse_index = select_random_verse_index(&exercise.verses);
    if let Some(ref mut selected_verse) = exercise.verses.get_mut(selected_verse_index) {
        selected_verse.verse.sourate = Some(sourate_name_from_verse(dbs, &selected_verse.verse));
    }

    let exclude_value = Some(Alternative { 
        content: exercise.verses[selected_verse_index].ungrouped_text.discriminant.clone().unwrap_or_default(), 
        verse: Some(exercise.verses[selected_verse_index].verse.clone())
    });

    // Convert discriminants (assuming they are chapter names) to the Alternative format
    let mut alternatives = extract_and_shuffle_options(&mut exercise.verses, 
    |statement| {
        let mut alternative = Alternative { 
            content: statement.ungrouped_text.discriminant.clone().unwrap_or_default(), 
            verse: Some(statement.verse.clone())
        };
        
        if let Some(ref mut verse) = alternative.verse {
            verse.text = None; // Set text to None in verse
        }
        
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

    let selected_verse = exercise.verses.get_mut(selected_verse_index).unwrap();
    let selected_discriminant = selected_verse.ungrouped_text.discriminant.take();   

    if let Some(discr) = selected_discriminant {
        alternatives.push(Alternative { content: discr, verse: Some(selected_verse.verse.clone()) });
    }
    alternatives.shuffle(&mut rand::thread_rng());
    
    Some(Exercise {
        statement: selected_verse.clone(),
        alternatives, // Pass the cloned alternatives here
        exercise_type: ExerciseType::A
    })
}
