use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::db::exercise::get_solution::get_solution;
use crate::db::similar::similars_by_key_count::get_count_for_kalima;
use crate::models::{ExerciseType, VerseUngrouped, Database};

pub fn generate_one(dbs: &Database, kalima: String, exercise_type: ExerciseType) -> Option<(VerseUngrouped, Vec<String>, ExerciseType)> {
    match exercise_type {
        ExerciseType::A => generate_A(dbs, kalima),
        ExerciseType::B => generate_B(dbs, kalima),
    }
}

pub fn generate_exercises(dbs: &Database, kalima: &str) -> Vec<(VerseUngrouped, Vec<String>, ExerciseType)> {
    let mut exercises = Vec::new();
    
    // Get the count for the given kalima
    let count = get_count_for_kalima(dbs, kalima);

    println!("Count {:?} {:?}", kalima, &count);
    
    // Compute the number of exercises to generate based on the count.
    // You can define your logic here. For simplicity, I'm using a formula
    // that generates exercises up to a max of 4 or one less than the total count, whichever is smaller.
    let num_exercises = std::cmp::min(4, count.saturating_sub(1));

    // Generate exercises of type A
    for _ in 0..num_exercises {
        if let Some(exercise) = generate_one(dbs, kalima.to_string(), ExerciseType::A) {
            exercises.push(exercise);
        }
    }

    // Generate exercises of type B
    for _ in 0..num_exercises {
        if let Some(exercise) = generate_one(dbs, kalima.to_string(), ExerciseType::B) {
            exercises.push(exercise);
        }
    }

    exercises
}


// fn select_random_verse<'a>(verses: &'a mut Vec<VerseUngrouped>) -> &'a mut VerseUngrouped {
//     let selected_verse_index = rand::thread_rng().gen_range(0..verses.len());
//     &mut verses[selected_verse_index]
// }

fn extract_and_shuffle_options<F>(verses: &[VerseUngrouped], 
                                  extractor: F, 
                                  exclude: &Option<String>) -> Vec<String>
where F: Fn(&VerseUngrouped) -> Option<String> {
    let options: Vec<String> = verses.iter()
        .filter_map(|verse| {
            let opt = extractor(verse);
            if opt.as_ref() != exclude.as_ref() {
                opt
            } else {
                None
            }
        })
        .collect();
    
    let unique_options: HashSet<_> = options.into_iter().collect();
    let mut unique_options: Vec<_> = unique_options.into_iter().collect();
    unique_options.truncate(2);
    let mut rng = rand::thread_rng();
    unique_options.shuffle(&mut rng);
    
    unique_options
}

// Instead of getting a mutable reference to the verse, just get its index
fn select_random_verse_index(verses: &Vec<VerseUngrouped>) -> usize {
    rand::thread_rng().gen_range(0..verses.len())
}

pub fn generate_A(dbs: &Database, kalima: String) -> Option<(VerseUngrouped, Vec<String>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }
    
    let exercise = &mut exercises[0];
    // let selected_verse = select_random_verse(&mut exercise.verses);

    let selected_verse_index = select_random_verse_index(&exercise.verses);

    // Get other discriminants/chapter names before mutating the selected verse
    let mut other_discriminants = extract_and_shuffle_options(&exercise.verses, 
        |verse| verse.discriminant.clone(), 
        &exercise.verses[selected_verse_index].discriminant);

    // Now, you can mutate the selected verse
    let selected_verse = exercise.verses.get_mut(selected_verse_index).unwrap();
    let selected_discriminant = selected_verse.discriminant.take();   
 
    if let Some(discr) = selected_discriminant {
        other_discriminants.push(discr);
    }
    
    Some((selected_verse.clone(), other_discriminants, ExerciseType::A))
}

pub fn generate_B(dbs: &Database, kalima: String) -> Option<(VerseUngrouped, Vec<String>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }
    
    let exercise = &mut exercises[0];

    // Step 1: Get the index of the random verse
    let selected_verse_index = select_random_verse_index(&exercise.verses);
    
    // Step 2: Extract the selected chapter name
    let selected_chapter_name = exercise.verses[selected_verse_index].chapter_name.clone();

    // Step 3: Extract and shuffle other chapter names
    let mut other_chapter_names = extract_and_shuffle_options(&exercise.verses, 
        |verse| Some(verse.chapter_name.clone()), 
        &Some(selected_chapter_name.clone()));

    // Step 4: Clear the chapter_name of the selected verse
    let selected_verse = &mut exercise.verses[selected_verse_index];
    selected_verse.chapter_name.clear();

    // Step 5: Add the selected_chapter_name back to the list of chapter names
    other_chapter_names.push(selected_chapter_name);

    Some((selected_verse.clone(), other_chapter_names, ExerciseType::B))
}
