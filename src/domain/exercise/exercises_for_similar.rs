use crate::models::{Similar, ExerciseOutput, Statement};
use crate::models::Database;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;

use crate::utils::extract_parts::extract_parts;

pub fn create(dbs: &Database, similar: &Similar) -> ExerciseOutput {
    let similar_db = &dbs.similar_db;

    let mut all_verses = Vec::new();
    for verse in &similar.verses {
        let mut modified_verse = verse.clone();

        let (pre, discriminant, post) = extract_parts(&verse.text);

        modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));

        all_verses.push(Statement {
            verse: modified_verse,
            pre,
            discriminant,
            post,
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

                        let (pre, discriminant, post) = extract_parts(&verse.text);
                        all_verses.push(Statement {
                            verse: verse.clone(),
                            pre,
                            discriminant,
                            kalima: kalima.clone(),
                            post,
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