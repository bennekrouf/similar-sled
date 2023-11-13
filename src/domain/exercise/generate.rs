use crate::domain::exercise::get_solution::get_solution;
use crate::models::{ExerciseType, Database, Exercise};
use crate::domain::exercise::generate_one_exercise::generate_one_exercise;

pub fn generate(dbs: &Database, exercise_type: ExerciseType, ranges: &Option<Vec<(u8, u8)>>) -> Vec<Exercise> {
    let mut solutions = get_solution(dbs, ranges);
    if solutions.is_empty() { return Vec::new(); }

    solutions.iter_mut().map(|solution| {
        generate_one_exercise(dbs, solution, exercise_type.clone())
    }).collect()
}

