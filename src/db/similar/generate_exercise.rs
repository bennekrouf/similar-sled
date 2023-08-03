use rand::Rng;
use crate::db::similar::similars_solutions::get_solution;
use crate::domain::coran::models::VerseUngrouped;
use crate::models::Database;

pub fn generate_exercise(dbs: &Database, similar_key: &str) -> Option<(VerseUngrouped, Vec<Option<String>>)> {
    let mut exercises = get_solution(dbs, similar_key);
    
    if exercises.is_empty() {
        return None;
    }
    
    // Get a random ExerciseOutput
    let exercise = &mut exercises[0]; // assume we always have at least one ExerciseOutput

    // Get a random VerseUngrouped and hide its discriminant
    let selected_verse_index = rand::thread_rng().gen_range(0..exercise.verses.len());

    // Temporarily remove the selected verse
    let mut selected_verse = exercise.verses.remove(selected_verse_index);
    let selected_discriminant = selected_verse.discriminant.take(); // hide the discriminant

    // Extract discriminants of the other verses
    let mut other_discriminants: Vec<Option<String>> = exercise.verses.iter()
        .filter(|verse| verse.discriminant.is_some() && verse.discriminant != selected_discriminant) 
        .map(|verse| verse.discriminant.clone()) // clone the discriminant
        .collect();
    other_discriminants.sort();
    other_discriminants.dedup(); // remove duplicates
    
    // Add the selected verse back into the exercise
    exercise.verses.insert(selected_verse_index, selected_verse.clone());

    Some((selected_verse, other_discriminants))
}