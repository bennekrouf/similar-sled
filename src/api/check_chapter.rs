use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::models::Database;
use crate::db::exercise::check_chapter;

#[get("/check_chapter?<kalima>&<selected_chapter_no>&<ayah>")]
pub fn check_chapter(
    kalima: String,
    selected_chapter_no: u32,
    ayah: u32,
    dbs: State<Database>,
) -> Json<(bool, Option<String>, Option<String>, Option<String>)> {
    let result = check_chapter::check_chapter(
        &dbs,
        kalima,
        selected_chapter_no,
        ayah,
    );
    Json(result)
}
