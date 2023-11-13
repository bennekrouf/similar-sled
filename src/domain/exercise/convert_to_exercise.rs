use crate::models::{Similar, ExerciseOutput, Statement, VerseOutput};
use crate::models::Database;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;

pub fn convert_to_exercise(dbs: &Database, similar: &Similar) -> ExerciseOutput {
    let similar_db = &dbs.similar_db;

    let verses_from_similar = similar.verses.iter().map(|verse| create_statement(dbs, verse, &similar.kalima, has_opposites(&similar.opposites)));

    let verses_from_opposites = similar.opposites.iter()
    .flat_map(|opposites| opposites.iter())
    .filter_map(|kalima| {
        let similar_from_db = get_similar_from_db(similar_db, kalima);
        similar_from_db.map(|s| (s.verses.clone(), kalima.clone(), s.opposites.clone()))  // Clone the necessary data
    })
    .flat_map(|(verses, kalima, opposites)| {
        verses.into_iter().map(move |verse| {
            create_statement(dbs, &verse, &kalima, has_opposites(&opposites))
        })
    });

    let all_verses = verses_from_similar.chain(verses_from_opposites).collect();

    ExerciseOutput {
        kalima: similar.kalima.clone(),
        verses: all_verses,
    }
}

fn create_statement(dbs: &Database, verse: &VerseOutput, kalima: &str, has_opposites: bool) -> Statement {
    let mut modified_verse = verse.clone();
    modified_verse.sourate = Some(sourate_name_from_verse(dbs, verse));

    Statement {
        verse: modified_verse,
        kalima: kalima.to_string(),
        has_opposites,
    }
}

fn has_opposites(opposites: &Option<Vec<String>>) -> bool {
    opposites.as_ref().map_or(false, |o| !o.is_empty())
}

fn get_similar_from_db(similar_db: &sled::Db, kalima: &str) -> Option<Similar> {
    similar_db.get(kalima).ok().flatten().and_then(|data| bincode::deserialize::<Similar>(&data).ok())
}
