use crate::models::VerseOutput;
use crate::models::Database;
use crate::domain::chapter::chapter;

pub fn sourate_name_from_verse(dbs: &Database, verse: &VerseOutput) -> String {
    // Some logic to get the sourate name from the verse chapter
    let sourate = chapter::get(dbs, verse.chapter_no as u8);
    match sourate {
        Ok(Some(sourate)) => sourate,
        Ok(None) | Err(_) => String::from("No found"),
    }
}
