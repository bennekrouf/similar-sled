use crate::models::{Database, UngroupedText};
use crate::domain::exercise::get_solution::get_solution;

pub fn check_chapter(
    dbs: &Database, 
    kalima: String,
    selected_chapter_no: u32,  // user's selected chapter_no
    verse_no: u32,
    discriminant: String,
) -> (bool, UngroupedText) {
    let solutions = get_solution(dbs, &kalima);
    println!("DISC : {:?}", kalima);

    // Check if the user's selection is correct
    for exercise in &solutions {
        for statement in &exercise.verses {
            if let Some(ref verse_discriminant) = statement.verse.ungrouped_text.as_ref().unwrap_or(&UngroupedText::default()).discriminant {
                if verse_discriminant == &discriminant && statement.verse.verse_no == verse_no && statement.verse.chapter_no == selected_chapter_no {
                    return (true, UngroupedText { pre: None, post: None, discriminant: None}); // Correctly matched
                }
            }
        }
    }

    // If we reached here, the user's selection was wrong
    // Let's find the corresponding verse to return its pre, post, and discriminant
    for exercise in &solutions {
        for statement in &exercise.verses {
            if statement.verse.verse_no == verse_no {
                return (
                    false,
                    statement.verse.ungrouped_text.as_ref().unwrap_or(&UngroupedText::default()).clone()
                );
            }
        }
    }

    // If we reached here, we didn't even find a verse with matching verse_no
    (false, UngroupedText { pre: None, post: None, discriminant: None})
}
