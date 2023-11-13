use rand::seq::SliceRandom;

use crate::domain::exercise::generate::generate;
use crate::models::{ExerciseType, Database, Exercise};

pub fn get_exercises(dbs: &Database, ranges: &Option<Vec<(u8, u8)>>) -> Vec<Exercise> {
    let mut exercises = Vec::new();
    
    let sourate_exercises = generate(dbs, ExerciseType::FindSourate, ranges);
    let discriminant_exercises = generate(dbs, ExerciseType::FindDiscriminant, ranges);

    exercises.extend(sourate_exercises.iter().cloned());
    exercises.extend(discriminant_exercises.iter().cloned());

     // Now shuffle the exercises
    let mut rng = rand::thread_rng();
    exercises.shuffle(&mut rng);

    exercises
}