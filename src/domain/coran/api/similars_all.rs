use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::domain::coran::models::{Chapter, ExerciseOutput, VerseUngrouped};
use crate::models::Database;
use crate::db::similar::similars_by_chapter;
use crate::db::similar::similars_solutions;
use crate::db::similar::check_discriminant;
use crate::db::similar::generate_exercise::generate_exercise;

#[get("/check_discriminant?<kalima>&<discriminant>&<ayah>&<chapter>")]
pub fn check_discriminant(
    kalima: String,
    discriminant: Option<String>,
    ayah: u32,
    chapter: u32,
    dbs: State<Database>,
) -> Json<bool> {
    let is_match = check_discriminant::check_discriminant(
        &dbs,
        kalima,
        discriminant,
        ayah,
        chapter,
    );
    Json(is_match)
}

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(kalima: String, dbs: State<Database>) -> Option<Json<(VerseUngrouped, Vec<String>)>> {
    generate_exercise(&dbs, kalima).map(Json)
}

#[get("/solutions/<kalima>")]
pub fn get_solutions(kalima: String, dbs: State<Database>) -> Json<Vec<ExerciseOutput>> {
    let solutions = similars_solutions::get_solution(&dbs, &kalima);
    Json(solutions)
}

#[get("/chapters")]
pub fn get_chapters(dbs: State<Database>) -> Json<Vec<Chapter>> {
    let chapters: Vec<Chapter> = dbs.chapter_db
        .iter()
        .filter_map(|result| match result {
            Ok((_, value)) => {
                let chapter: Chapter = bincode::deserialize(&value).unwrap();
                let similar_objects = similars_by_chapter::get(&dbs, chapter.no as u32);
                let count = similar_objects.len() as u32;
                if count > 0 {
                    Some(Chapter { count: Some(count), ..chapter })
                } else {
                    None
                }
            },
            _ => None,
        })
        .collect();

    Json(chapters)
}