use rocket::serde::json::Json;
use rocket::{get, State};

use crate::db::exercise::check_discriminant as check_discriminant_db;
use crate::models::Database;

#[get("/check_discriminant?<kalima>&<discriminant>&<ayah>&<chapter_no>")]
pub fn check_discriminant_endpoint(
    kalima: String,
    discriminant: Option<String>,
    ayah: u32,
    chapter_no: u32,
    dbs: &State<Database>,
) -> Json<(bool, String)> {
    let is_match =
        check_discriminant_db::check_discriminant(dbs, kalima, discriminant, ayah, chapter_no);
    Json(is_match)
}

