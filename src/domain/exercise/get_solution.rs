use crate::models::{Similar, ExerciseOutput};
use crate::models::Database;

use crate::domain::exercise::sort_exercises::sort_exercises;
use crate::domain::exercise::exercises_for_similar::create;
use crate::utils::is_in_range::is_in_range;

pub fn get_solution(dbs: &Database, ranges: &Option<Vec<(u8, u8)>>) -> Vec<ExerciseOutput> {
    let similar_db = &dbs.similar_db;

    let mut solutions: Vec<ExerciseOutput> = similar_db
        .iter()
        .filter_map(|result| {
            let (_, value) = result.ok()?;
            // Deserialize the value into Similar
            let similar: Similar = bincode::deserialize(&value).ok()?;

            // Check if any verse of this Similar is in the specified range
            if similar.verses.iter().any(|verse| is_in_range(&verse.chapter_no, &ranges)) {
                // Convert Similar to ExerciseOutput if it passes the range and opposites check
                let exercise = create(dbs, &similar, ranges);
                Some(exercise)
            } else {
                None
            }
        })
        .collect();

    sort_exercises(&mut solutions);

    solutions
}
