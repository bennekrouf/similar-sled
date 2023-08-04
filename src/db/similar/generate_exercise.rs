use std::collections::HashSet;

use rand::Rng;
use crate::db::similar::similars_solutions::get_solution;
use crate::domain::coran::models::VerseUngrouped;
use crate::models::Database;

pub fn generate_exercise(dbs: &Database, similar_key: String) -> Option<(VerseUngrouped, Vec<String>)> {
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
    let _selected_kalima = selected_verse.kalima.clear(); // hide the kalima

    // Extract kalimas of the other verses
    let other_discriminants: Vec<String> = exercise.verses.iter()
        .filter(|verse| verse.discriminant.is_some() && verse.discriminant != selected_discriminant)
        .map(|verse| verse.kalima.clone()) // clone the kalima
        .collect();

    // Convert to HashSet to remove duplicates, and then back to Vec
    let other_discriminants: Vec<String> = other_discriminants.into_iter().collect::<HashSet<_>>().into_iter().collect();

    
    // Add the selected verse back into the exercise
    exercise.verses.insert(selected_verse_index, selected_verse.clone());

    Some((selected_verse, other_discriminants))
}