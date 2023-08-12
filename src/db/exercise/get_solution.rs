use crate::models::{Similar, ExerciseOutput, VerseUngrouped};
use crate::models::Database;
use crate::utils::extract_parts::extract_parts;
use crate::db::chapter::chapter_name;

pub fn get_solution(dbs: &Database, kalima: &String) -> Vec<ExerciseOutput> {
    let similar_db = &dbs.similar_db;

    let mut solutions: Vec<ExerciseOutput> = similar_db
        .iter()
        .filter_map(|result| {
            let (key, value) = result.ok()?;
            let key = std::str::from_utf8(&key).ok()?;
            if key == kalima.clone() {
                // Deserialize the value into Similar
                let similar: Similar = bincode::deserialize(&value).ok()?;

                // Convert Similar to ExerciseOutput
                let exercise = convert_to_exercise(dbs, &similar);
                Some(exercise)
            } else {
                None
            }
        })
        .collect();

    sort_exercises(&mut solutions);

    solutions
}

pub fn convert_to_exercise(dbs: &Database, similar: &Similar) -> ExerciseOutput {
    let similar_db = &dbs.similar_db;

    let mut all_verses = Vec::new();
    for verse in &similar.verses {
        let (pre, discriminant, post) = extract_parts(&verse.text);

        let chapter_name_result = chapter_name::get(dbs, verse.chapter_no as u8);
        let chapter_name = match chapter_name_result {
                Ok(Some(name)) => name,
                Ok(None) | Err(_) => String::from("No found"),
            };

        all_verses.push(VerseUngrouped {
            // text: verse.text.clone(),
            pre,
            discriminant,
            post,
            kalima: similar.kalima.clone(),
            chapter_name,
            ayah: verse.ayah,
            chapter_no: verse.chapter_no,
            has_opposites: match &similar.opposite_similars {
                Some(opposite_similars) => !opposite_similars.is_empty(),
                None => false,
            },
        });
    }

    if let Some(opposite_similars) = &similar.opposite_similars {
        for kalima in opposite_similars {
            if let Ok(Some(data)) = similar_db.get(kalima) {
                if let Ok(similar) = bincode::deserialize::<Similar>(&data) {
                    for verse in &similar.verses {

                        let chapter_name_result = chapter_name::get(dbs, verse.chapter_no as u8);
                        let chapter_name = match chapter_name_result {
                                Ok(Some(name)) => name,
                                Ok(None) | Err(_) => String::from("No found"),
                            };

                        let (pre, discriminant, post) = extract_parts(&verse.text);
                        all_verses.push(VerseUngrouped {
                            pre,
                            discriminant,
                            kalima: kalima.clone(),
                            chapter_name,
                            post,
                            ayah: verse.ayah,
                            chapter_no: verse.chapter_no,
                            has_opposites: !similar.opposite_similars.clone().unwrap().is_empty(),
                        });
                    }
                }
            }
        }
    }

    ExerciseOutput {
        kalima: similar.kalima.clone(),
        verses: all_verses,
    }
}

pub fn sort_exercises(solutions: &mut [ExerciseOutput]) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    for exercise in solutions {
        exercise.verses.shuffle(&mut rng);
    }
}