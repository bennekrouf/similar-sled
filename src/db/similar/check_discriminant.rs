use crate::models::Database;
use crate::db::similar::similars_solutions::get_solution;

pub fn check_discriminant(
    dbs: &Database, 
    // similar_key: &str, 
    kalima: String, 
    ayah: u32, 
    chapter: u32
) -> bool {
    let solutions = get_solution(dbs, kalima.clone());

    for exercise in solutions {
        if exercise.kalima != kalima {
            continue;
        }

        for verse in exercise.verses {
            if verse.ayah != ayah {
                continue;
            }

            if verse.kalima != kalima {
                continue;
            }

            if verse.chapter != chapter {
                continue;
            }

            // All fields matched, return true
            return true;
        }
    }

    // No match found in any exercise
    false
}