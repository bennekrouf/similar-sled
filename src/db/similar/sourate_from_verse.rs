use crate::models::VerseOutput;
use crate::models::Database;
use crate::db::chapter::chapter;

pub fn sourate_name_from_verse(dbs: &Database, verse: &VerseOutput) -> String {
    // Some logic to get the sourate name from the verse chapter
    let chapter_name_result = chapter::get(dbs, verse.chapter_no as u8);
    match chapter_name_result {
        Ok(Some(name)) => name,
        Ok(None) | Err(_) => String::from("No found"),
    }
}
