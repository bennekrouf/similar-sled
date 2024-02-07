use crate::models::exercise_output::ExerciseOutput;

pub fn sort_exercises(solutions: &mut [ExerciseOutput]) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    for exercise in solutions {
        exercise.verses.shuffle(&mut rng);
    }
}