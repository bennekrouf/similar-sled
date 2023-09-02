use crate::models::{Similar, ExerciseOutput, Statement};
use crate::models::Database;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;

use crate::utils::extract_parts::extract_parts;

pub fn create(dbs: &Database, similar: &Similar) -> ExerciseOutput {
    let similar_db = &dbs.similar_db;

    let mut all_verses = Vec::new();
    for verse in &similar.verses {
        let mut modified_verse = verse.clone();

        let ungrouped_text = extract_parts(verse.text.as_ref().map(String::as_str));

        modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));

        all_verses.push(Statement {
            verse: modified_verse,
            ungrouped_text,
            kalima: similar.kalima.clone(),
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
                        let mut modified_verse = verse.clone();
                        modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));

                        let ungrouped_text = extract_parts(verse.text.as_ref().map(String::as_str));
                        all_verses.push(Statement {
                            verse: verse.clone(),
                            ungrouped_text,
                            kalima: kalima.clone(),
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