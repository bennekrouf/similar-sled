use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::db::exercise::get_solution::get_solution;
use crate::db::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::db::exercise::select_random_verse_index::select_random_verse_index;
use crate::models::{ExerciseType, Statement, Database, Alternative};

pub fn generate(dbs: &Database, kalima: String) -> Option<(Statement, Vec<Alternative>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }
    
    let exercise = &mut exercises[0];
    let selected_verse_index = select_random_verse_index(&exercise.verses);

    let exclude_value = Some(Alternative { 
        content: exercise.verses[selected_verse_index].discriminant.clone().unwrap_or_default(), 
        ayah: None
    });

    // Convert discriminants (assuming they are chapter names) to the Alternative format
    let alternatives = extract_and_shuffle_options(&mut exercise.verses, 
        |statement| Some(Alternative { 
            content: statement.discriminant.clone().unwrap_or_default(), 
            ayah: None  // As this is for generate_A
        }),
        &exclude_value
    );

    // Deduplicate the list using HashSet
    let mut distinct_set: HashSet<_> = alternatives.into_iter().collect();
    
    // Convert HashSet back to Vec and shuffle
    let mut distinct_vec: Vec<_> = distinct_set.drain().collect();
    distinct_vec.shuffle(&mut rand::thread_rng());

    // Limit to 3 possible answers (minus the correct answer which we will add later)
    distinct_vec.truncate(2);

    let selected_verse = exercise.verses.get_mut(selected_verse_index).unwrap();
    let selected_discriminant = selected_verse.discriminant.take();   

    let mut final_chapter_ayahs = distinct_vec;
    if let Some(discr) = selected_discriminant {
        final_chapter_ayahs.push(Alternative { content: discr, ayah: None });
    }
    
    Some((selected_verse.clone(), final_chapter_ayahs, ExerciseType::A))
}