use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::db::exercise::get_solution::get_solution;
use crate::db::similar::similars_by_key_count::get_count_for_kalima;
use crate::models::{ExerciseType, VerseUngrouped, Database, ChapterAyah};

pub fn generate_one(dbs: &Database, kalima: String, exercise_type: ExerciseType) -> Option<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)> {
    match exercise_type {
        ExerciseType::A => generate_A(dbs, kalima),
        ExerciseType::B => generate_B(dbs, kalima),
    }
}

pub fn generate_exercises(dbs: &Database, kalima: &str) -> Vec<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)> {
    let mut exercises = Vec::new();
    
    // Get the count for the given kalima
    let count = get_count_for_kalima(dbs, kalima);
    println!("Count {:?} {:?}", kalima, &count);
    
    // Compute the number of exercises to generate based on the count.
    let num_exercises = std::cmp::min(4, count.saturating_sub(1));

    // HashSet to keep track of seen exercises
    let mut seen_exercises = HashSet::new();

    // Function to attempt generating a unique exercise with a retry limit
    fn attempt_generate(dbs: &Database, kalima: String, ex_type: ExerciseType, seen: &mut HashSet<(VerseUngrouped, Vec<ChapterAyah>)>) -> Option<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)> {
        const MAX_RETRIES: usize = 10;
        for _ in 0..MAX_RETRIES {
            if let Some(exercise) = generate_one(dbs, kalima.clone(), ex_type.clone()) {
                let (verse, discriminant, _) = &exercise;
                if seen.insert((verse.clone(), discriminant.clone())) {
                    return Some(exercise);
                }
            }
        }
        None
    }

    // Generate exercises of type A
    for _ in 0..num_exercises {
        if let Some(exercise) = attempt_generate(dbs, kalima.to_string(), ExerciseType::A, &mut seen_exercises) {
            exercises.push(exercise);
        }
    }

    // Generate exercises of type B
    for _ in 0..num_exercises {
        if let Some(exercise) = attempt_generate(dbs, kalima.to_string(), ExerciseType::B, &mut seen_exercises) {
            exercises.push(exercise);
        }
    }

     // Now shuffle the exercises
    let mut rng = rand::thread_rng();
    exercises.shuffle(&mut rng);

    exercises
}


fn extract_and_shuffle_options<T: PartialEq>(
    verses: &[VerseUngrouped],
    mut extractor: impl FnMut(&VerseUngrouped) -> Option<T>, // change to FnMut here
    exclude: &Option<T>
) -> Vec<T> {
    let mut options: Vec<T> = verses.iter()
        .filter_map(|verse| {
            let option = extractor(verse);  // since we have changed it to FnMut, it's mutable now
            if let Some(exclude_value) = exclude {
                if option.as_ref() == Some(&exclude_value) {
                    return None;
                }
            }
            option
        })
        .collect();

    options.shuffle(&mut rand::thread_rng());
    options
}

// Instead of getting a mutable reference to the verse, just get its index
fn select_random_verse_index(verses: &Vec<VerseUngrouped>) -> usize {
    rand::thread_rng().gen_range(0..verses.len())
}

pub fn generate_A(dbs: &Database, kalima: String) -> Option<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }
    
    let exercise = &mut exercises[0];
    let selected_verse_index = select_random_verse_index(&exercise.verses);

    // Convert discriminants (assuming they are chapter names) to the ChapterAyah format
    let other_chapter_ayahs = extract_and_shuffle_options(&exercise.verses, 
        |verse| Some(ChapterAyah { 
            chapter_name: verse.discriminant.clone().unwrap_or_default(), 
            ayah: None  // As this is for generate_A
        }),
        &Some(ChapterAyah { 
            chapter_name: exercise.verses[selected_verse_index].discriminant.clone().unwrap_or_default(), 
            ayah: None
        })
    );

    // Deduplicate the list using HashSet
    let mut distinct_set: HashSet<_> = other_chapter_ayahs.into_iter().collect();
    
    // Convert HashSet back to Vec and shuffle
    let mut distinct_vec: Vec<_> = distinct_set.drain().collect();
    distinct_vec.shuffle(&mut rand::thread_rng());

    // Limit to 3 possible answers (minus the correct answer which we will add later)
    distinct_vec.truncate(2);

    let selected_verse = exercise.verses.get_mut(selected_verse_index).unwrap();
    let selected_discriminant = selected_verse.discriminant.take();   

    let mut final_chapter_ayahs = distinct_vec;
    if let Some(discr) = selected_discriminant {
        final_chapter_ayahs.push(ChapterAyah { chapter_name: discr, ayah: None });
    }
    
    Some((selected_verse.clone(), final_chapter_ayahs, ExerciseType::A))
}

pub fn generate_B(dbs: &Database, kalima: String) -> Option<(VerseUngrouped, Vec<ChapterAyah>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }

    let exercise = &mut exercises[0];

    // Step 1: Get the index of the random verse
    let selected_verse_index = select_random_verse_index(&exercise.verses);
    
    // Step 2: Extract the selected chapter name and ayah
    let selected_chapter_name = exercise.verses[selected_verse_index].chapter_name.clone();
    let selected_ayah = exercise.verses[selected_verse_index].ayah;

    // Step 3: Extract and shuffle other chapter names and ayahs
    let mut other_chapter_data_set = std::collections::HashSet::new();
    let mut other_chapter_data = extract_and_shuffle_options(&exercise.verses, 
        |verse| {
            let chapter_ayah = Some(ChapterAyah {
                chapter_name: verse.chapter_name.clone(),
                ayah: Some(verse.ayah)
            });
            if other_chapter_data_set.insert(chapter_ayah.clone()) {
                chapter_ayah
            } else {
                None
            }
        }, 
        &Some(ChapterAyah {
            chapter_name: selected_chapter_name.clone(),
            ayah: Some(selected_ayah)
        }));
    
    // Limit to 3 possible answers (minus the correct answer which we will add back in Step 5)
    other_chapter_data.truncate(2);

    // Step 4: Hide the chapter_name, chapter_no, and ayah of the selected verse
    let selected_verse = &mut exercise.verses[selected_verse_index];
    selected_verse.chapter_name.clear();
    selected_verse.chapter_no = 0; // You can set this to any default value
    selected_verse.ayah = 0; // Set it to a default value

    // Step 5: Add the selected chapter data back to the list
    other_chapter_data.push(ChapterAyah {
        chapter_name: selected_chapter_name,
        ayah: Some(selected_ayah),
    });

    Some((selected_verse.clone(), other_chapter_data, ExerciseType::B))
}
