use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::domain::coran::db::exercise::get_solution::get_solution;
use crate::domain::coran::models::VerseUngrouped;
use crate::models::Database;

pub fn generate(dbs: &Database, kalima: String) -> Option<(VerseUngrouped, Vec<String>)> {
    let mut exercises = get_solution(dbs, &kalima);
    
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
    // let _selected_kalima = selected_verse.kalima.clear(); // hide the kalima

    // Extract discriminants from verses, handling whether or not verse has opposites
    let other_discriminants: Vec<String> = exercise.verses.iter()
    .filter_map(|verse| {
        match &verse.discriminant {
            Some(discriminant) => {
                if Some(discriminant) != selected_discriminant.as_ref() {
                    Some(discriminant.clone())
                } else {
                    None
                }
            }
            None => None,
        }
    })
    .collect();

    // Convert to HashSet to remove duplicates
    let other_discriminants: HashSet<_> = other_discriminants.into_iter().collect();

    // Convert back to Vec
    let mut other_discriminants: Vec<_> = other_discriminants.into_iter().collect();

    // Add the selected verse back into the exercise
    exercise.verses.insert(selected_verse_index, selected_verse.clone());
    // Ensure the list has a maximum of 2 items
    other_discriminants.truncate(2);

    // Add the selected_discriminant to the list
    if let Some(discr) = selected_discriminant {
        other_discriminants.push(discr);
    }

    // Shuffle the other_discriminants vector
    let mut rng = rand::thread_rng();
    other_discriminants.shuffle(&mut rng);

    Some((selected_verse, other_discriminants))
}