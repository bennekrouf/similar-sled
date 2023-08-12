use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::models::Database;
use crate::db::exercise::check_discriminant;

#[get("/check_discriminant?<kalima>&<discriminant>&<ayah>&<chapter_no>")]
pub fn check_discriminant(
    kalima: String,
    discriminant: Option<String>,
    ayah: u32,
    chapter_no: u32,
    dbs: State<Database>,
) -> Json<(bool, String)> {
    let is_match = check_discriminant::check_discriminant(
        &dbs,
        kalima,
        discriminant,
        ayah,
        chapter_no,
    );
    Json(is_match)
}
