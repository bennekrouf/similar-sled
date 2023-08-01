use crate::domain::coran::models::{SimilarOutput, Verse, VerseOutput};
use crate::models::Database;
use crate::db::verse::verse_by_chapter_and_ayat;
use crate::db::chapter::chapter_name;

pub fn create_similar_output(kalima: String, verses: &Vec<String>, dbs: &Database) -> SimilarOutput {
    let verses: Vec<VerseOutput> = verses
        .iter()
        .map(|reference| {
            let split: Vec<&str> = reference.split(":").collect();
            let chapter: u32 = split[0].parse().expect("Not a valid u32");
            let ayah: u32 = split[1].parse().expect("Not a valid u32");

            let chapter_name_result = chapter_name::get(dbs, chapter as u8);
            let chapter_name = match chapter_name_result {
                Ok(Some(name)) => name,
                Ok(None) | Err(_) => String::from("Default Chapter"),
            };

            let text = match verse_by_chapter_and_ayat::get(dbs, chapter, ayah) {
                Ok(Some(verse_text)) => verse_text,
                Ok(None) => String::from("Verse not found"),
                Err(_) => String::from("Error retrieving verse"),
            };

            VerseOutput {
                sourate: chapter_name,
                chapter,
                verse: Verse {
                    text,
                    ayah,
                    chapter,
                },
            }
        })
        .collect();

    SimilarOutput { kalima, verses }
}