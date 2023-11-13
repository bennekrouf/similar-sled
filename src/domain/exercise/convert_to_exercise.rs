use crate::models::{Similar, ExerciseOutput, Statement};
use crate::models::Database;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;

pub fn convert_to_exercise(dbs: &Database, similar: &Similar) -> ExerciseOutput {
    let similar_db = &dbs.similar_db;

    let mut all_verses = Vec::new();
    for verse in &similar.verses {
        let mut modified_verse = verse.clone();

        modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));

        all_verses.push(Statement {
            verse: modified_verse,
            kalima: similar.kalima.clone(),
            has_opposites: match &similar.opposites {
                Some(opposites) => !opposites.is_empty(),
                None => false,
            },
        });
    }

    if let Some(opposites) = &similar.opposites {
        for kalima in opposites {
            if let Ok(Some(data)) = similar_db.get(kalima) {
                if let Ok(similar) = bincode::deserialize::<Similar>(&data) {
                    for verse in &similar.verses {
                        let mut modified_verse = verse.clone();
                        modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));

                        all_verses.push(Statement {
                            verse: verse.clone(),
                            kalima: kalima.clone(),
                            has_opposites: !similar.opposites.clone().unwrap().is_empty(),
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