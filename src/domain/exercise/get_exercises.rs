use std::collections::HashSet;
use rand::seq::SliceRandom;

use crate::domain::exercise::generate::generate as generate_A;

use crate::domain::similar::similars_by_key_count::get_count_for_kalima;
use crate::models::{ExerciseType, Database, Exercise};

pub fn get_exercises(dbs: &Database, kalima: &str) -> Vec<Exercise> {
    let mut exercises = Vec::new();
    
    // Get the count for the given kalima
    let count = get_count_for_kalima(dbs, kalima);
    println!("Count {:?} {:?}", kalima, &count);
    
    // Compute the number of exercises to generate based on the count.
    let num_exercises = std::cmp::min(1, count.saturating_sub(1));

    // HashSet to keep track of seen exercises
    let mut seen_exercises = HashSet::new();

    // Generate exercises of type FindDscriminant
    for _ in 0..num_exercises {
        if let Some(exercise) = attempt_generate(dbs, kalima.to_string(), ExerciseType::FindSourate, &mut seen_exercises) {
            exercises.push(exercise);
        }
    }

     // Now shuffle the exercises
    let mut rng = rand::thread_rng();
    exercises.shuffle(&mut rng);

    exercises
}

// Function to attempt generating a unique exercise with a retry limit
fn attempt_generate(
    dbs: &Database,
    kalima: String,
    ex_type: ExerciseType,
    seen: &mut HashSet<Exercise>,
) -> Option<Exercise> {
    const MAX_RETRIES: usize = 10;
    for _ in 0..MAX_RETRIES {
        if let Some(exercise) = generate_A(dbs, kalima.clone(), ex_type.clone()) {
            if seen.insert(exercise.clone()) {
                return Some(exercise);
            }
        }
    }
    None
}