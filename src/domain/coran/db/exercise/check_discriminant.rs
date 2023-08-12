use crate::models::Database;
use crate::domain::coran::db::exercise::get_solution::get_solution;

pub fn check_discriminant(
    dbs: &Database, 
    kalima: String,
    discriminant: Option<String>,
    ayah: u32, 
    chapter_no: u32
) -> (bool, String) {
    let solutions = get_solution(dbs, &kalima);
    
    // First loop to check if a match exists
    for exercise in &solutions {
        for verse in &exercise.verses {
            if verse.ayah == ayah && verse.chapter_no == chapter_no && verse.discriminant == discriminant {
                // Match found
                return (true, String::from(""));
            }
        }
    }
    
    // If we reached here, no match was found in the first loop
    // Let's now try to find the chapter name for the given ayah and chapter_no
    for exercise in &solutions {
        for verse in &exercise.verses {
            if verse.ayah == ayah && verse.chapter_no == chapter_no {
                // Return the chapter name of the matching verse
                return (false, verse.chapter_name.clone());
            }
        }
    }

    // If we reached here, we didn't even find a verse with matching ayah and chapter_no
    (false, String::from("Chapter name not found"))
}
