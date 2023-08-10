use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::domain::coran::models::{Chapter, ExerciseOutput, VerseUngrouped};
use crate::models::Database;
use crate::domain::coran::db::similar::similars_by_chapter;
use crate::domain::coran::db::exercise::get_solution;
use crate::domain::coran::db::exercise::check_discriminant;
use crate::domain::coran::db::exercise::find_discriminant;

#[get("/check_discriminant?<kalima>&<discriminant>&<ayah>&<chapter_no>")]
pub fn check_discriminant(
    kalima: String,
    discriminant: Option<String>,
    ayah: u32,
    chapter_no: u32,
    dbs: State<Database>,
) -> Json<bool> {
    let is_match = check_discriminant::check_discriminant(
        &dbs,
        kalima,
        discriminant,
        ayah,
        chapter_no,
    );
    Json(is_match)
}

#[get("/exercise/<kalima>")]
pub fn generate_exercise_endpoint(kalima: String, dbs: State<Database>) -> Option<Json<(VerseUngrouped, Vec<String>)>> {
   find_discriminant::generate(&dbs, kalima).map(Json)
}

#[get("/solutions/<kalima>")]
pub fn get_solutions(kalima: String, dbs: State<Database>) -> Json<Vec<ExerciseOutput>> {
    let solutions = get_solution::get_solution(&dbs, &kalima);
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