use crate::models::{Database, UngroupedText};
use crate::domain::exercise::get_solution::get_solution;

pub fn check_discriminant(
    dbs: &Database, 
    kalima: String,
    discriminant: Option<String>,
    verse_no: u32, 
    chapter_no: u32
) -> (bool, String) {
    let solutions = get_solution(dbs, &kalima);
    
    // First loop to check if a match exists
    for exercise in &solutions {
        for statement in &exercise.verses {
            if statement.verse.verse_no == verse_no && statement.verse.chapter_no == chapter_no && statement.verse.ungrouped_text.as_ref().unwrap_or(&UngroupedText::default()).discriminant == discriminant {
                // Match found
                return (true, String::from(""));
            }
        }
    }
    
    // If we reached here, no match was found in the first loop
    // Let's now try to find the chapter name for the given verse_no and chapter_no
    for exercise in &solutions {
        for statement in &exercise.verses {
            if statement.verse.verse_no == verse_no && statement.verse.chapter_no == chapter_no {
                // Return the chapter name of the matching verse
                return (false, statement.verse.sourate.as_ref().unwrap_or(&"".to_string()).clone());
            }
        }
    }

    // If we reached here, we didn't even find a verse with matching verse_no and chapter_no
    (false, String::from("Chapter name not found"))
}
