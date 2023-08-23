use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::db::exercise::a::generate as generate_A;
use crate::db::exercise::b::generate as generate_B;

use crate::db::similar::similars_by_key_count::get_count_for_kalima;
use crate::models::{ExerciseType, Statement, Database, Alternative};

pub fn generate_one(dbs: &Database, kalima: String, exercise_type: ExerciseType) -> Option<(Statement, Vec<Alternative>, ExerciseType)> {
    match exercise_type {
        ExerciseType::A => generate_A(dbs, kalima),
        ExerciseType::B => generate_B(dbs, kalima),
    }
}

pub fn generate_exercises(dbs: &Database, kalima: &str) -> Vec<(Statement, Vec<Alternative>, ExerciseType)> {
    let mut exercises = Vec::new();
    
    // Get the count for the given kalima
    let count = get_count_for_kalima(dbs, kalima);
    println!("Count {:?} {:?}", kalima, &count);
    
    // Compute the number of exercises to generate based on the count.
    let num_exercises = std::cmp::min(4, count.saturating_sub(1));

    // HashSet to keep track of seen exercises
    let mut seen_exercises = HashSet::new();

    // Function to attempt generating a unique exercise with a retry limit
    fn attempt_generate(dbs: &Database, kalima: String, ex_type: ExerciseType, seen: &mut HashSet<(Statement, Vec<Alternative>)>) -> Option<(Statement, Vec<Alternative>, ExerciseType)> {
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
    statements: &[Statement],
    mut extractor: impl FnMut(&Statement) -> Option<T>, // change to FnMut here
    exclude: &Option<T>
) -> Vec<T> {
    let mut options: Vec<T> = statements.iter()
        .filter_map(|statement| {
            let option = extractor(statement);  // since we have changed it to FnMut, it's mutable now
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
fn select_random_verse_index(statements: &Vec<Statement>) -> usize {
    rand::thread_rng().gen_range(0..statements.len())
}
