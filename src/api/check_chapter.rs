use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::models::{Database, UngroupedText};
use crate::domain::exercise::check_chapter;

#[get("/check_chapter?<kalima>&<selected_chapter_no>&<verse_no>&<discriminant>")]
pub fn check_chapter(
    kalima: String,
    selected_chapter_no: u32,
    verse_no: u32,
    discriminant: String,
    dbs: State<Database>,
) -> Json<(bool, Option<UngroupedText>)> {
    let result = check_chapter::check_chapter(
        &dbs,
        kalima,
        selected_chapter_no,
        verse_no,
        discriminant,
    );
    Json(result)
}
