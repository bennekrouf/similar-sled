use rand::seq::SliceRandom;

use crate::domain::exercise::get_solution::get_solution;
use crate::domain::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::domain::exercise::select_random_verse_index::select_random_verse_index;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::models::{ExerciseType, Statement, Database, Alternative};

use crate::utils::deduplicate_by_field::deduplicate_by_field;

pub fn generate(dbs: &Database, kalima: String) -> Option<(Statement, Vec<Alternative>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }
    
    let exercise = &mut exercises[0];
    let selected_verse_index = select_random_verse_index(&exercise.verses);
    if let Some(ref mut selected_verse) = exercise.verses.get_mut(selected_verse_index) {
        selected_verse.verse.sourate = Some(sourate_name_from_verse(dbs, &selected_verse.verse));
    }

    let exclude_value = Some(Alternative { 
        content: exercise.verses[selected_verse_index].discriminant.clone().unwrap_or_default(), 
        ayah: Some(exercise.verses[selected_verse_index].verse.clone())
    });

    // Convert discriminants (assuming they are chapter names) to the Alternative format
    let mut alternatives = extract_and_shuffle_options(&mut exercise.verses, 
        |statement| Some(Alternative { 
            content: statement.discriminant.clone().unwrap_or_default(), 
            ayah: Some(statement.verse.clone())
        }),
        &exclude_value
    );

    for alternative in &mut alternatives {
        if let Some(ref mut ayah) = alternative.ayah {
            ayah.sourate = Some(sourate_name_from_verse(dbs, ayah));
        }
    }

    let mut distinct_alternatives = deduplicate_by_field(alternatives, |alt| alt.content.clone());
    distinct_alternatives.shuffle(&mut rand::thread_rng());

    // Limit to 3 possible answers (minus the correct answer which we will add later)
    distinct_alternatives.truncate(2);

    let selected_verse = exercise.verses.get_mut(selected_verse_index).unwrap();
    let selected_discriminant = selected_verse.discriminant.take();   

    let mut final_alternatives = distinct_alternatives;
    if let Some(discr) = selected_discriminant {
        final_alternatives.push(Alternative { content: discr, ayah: Some(selected_verse.verse.clone()) });
    }
    
    Some((selected_verse.clone(), final_alternatives, ExerciseType::A))
}