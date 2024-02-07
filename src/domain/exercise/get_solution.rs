use crate::models::{similar::Similar, exercise_output::ExerciseOutput};
use crate::models::database::Database;

use crate::domain::exercise::sort_exercises::sort_exercises;
use crate::domain::exercise::exercises_for_similar::exercises_for_similar;
use crate::utils::is_chapter_in_range::is_chapter_in_range;
use crate::utils::is_verse_learnt::is_verse_learnt;
use crate::learning::models::analytic::Analytic;

pub fn get_solution(dbs: &Database, ranges: &Option<Vec<(u8, u8)>>, analytics: &Vec<Analytic>) -> Vec<ExerciseOutput> {
    let similar_db = &dbs.similar_db;

    let mut solutions: Vec<ExerciseOutput> = similar_db
        .iter()
        .filter_map(|result| {
            let (_, value) = result.ok()?;
            // Deserialize the value into Similar
            let similar: Similar = bincode::deserialize(&value).ok()?;

            // Check if any verse of this Similar is in the specified range
            if similar.verses.iter().any(|verse| 
                is_chapter_in_range(&verse.chapter_no, &ranges) 
                && !is_verse_learnt(verse.verse_no, analytics)) {
                // Convert Similar to ExerciseOutput if it passes the range and opposites check
                let exercise = exercises_for_similar(dbs, &similar, ranges);
                Some(exercise)
            } else {
                None
            }
        })
        .collect();

    sort_exercises(&mut solutions);

    solutions
}
