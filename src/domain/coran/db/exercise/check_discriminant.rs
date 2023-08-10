use crate::models::Database;
use crate::domain::coran::db::exercise::get_solution::get_solution;

pub fn check_discriminant(
    dbs: &Database, 
    kalima: String,
    discriminant: Option<String>,
    ayah: u32, 
    chapter_no: u32
) -> bool {
    let solutions = get_solution(dbs, &kalima);

    for exercise in solutions {
        for verse in exercise.verses {
            if verse.ayah != ayah {
                continue;
            }

            if verse.discriminant != discriminant {
                continue;
            }

            if verse.chapter_no != chapter_no {
                continue;
            }

            // All fields matched, return true
            return true;
        }
    }

    // No match found in any exercise
    false
}