use crate::models::Database;
use crate::db::exercise::get_solution::get_solution;
pub fn check_chapter(
    dbs: &Database, 
    kalima: String,
    selected_chapter_no: u32,  // user's selected chapter_no
    ayah: u32
) -> (bool, Option<String>, Option<String>, Option<String>) {
    let solutions = get_solution(dbs, &kalima);

    // Check if the user's selection is correct
    for exercise in &solutions {
        for verse in &exercise.verses {
            if verse.ayah == ayah && verse.chapter_no == selected_chapter_no {
                return (true, None, None, None); // Correctly matched
            }
        }
    }

    // If we reached here, the user's selection was wrong
    // Let's find the corresponding verse to return its pre, post, and discriminant
    for exercise in &solutions {
        for verse in &exercise.verses {
            if verse.ayah == ayah {
                return (
                    false, 
                    verse.pre.clone(), 
                    verse.post.clone(), 
                    verse.discriminant.clone()
                );
            }
        }
    }

    // If we reached here, we didn't even find a verse with matching ayah
    (false, None, None, None)
}
