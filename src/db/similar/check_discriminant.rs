use crate::models::Database;
use crate::db::similar::similars_solutions::get_solution;

pub fn check(
    dbs: &Database, 
    similar_key: &str, 
    kalima: Option<String>, 
    pre: Option<String>, 
    discriminant: Option<String>,
    post: Option<String>, 
    ayah: Option<u32>, 
    chapter: Option<u32>
) -> bool {
    let solutions = get_solution(dbs, similar_key);

    for exercise in solutions {
        if Some(exercise.kalima) != kalima {
            continue;
        }

        for verse in exercise.verses {
            if Some(verse.pre) != Some(pre.clone()) {
                continue;
            }

            if Some(verse.discriminant) != Some(discriminant.clone()) {
                continue;
            }

            if Some(verse.post) != Some(post.clone()) {
                continue;
            }

            if Some(verse.ayah) != ayah {
                continue;
            }

            if Some(verse.chapter) != chapter {
                continue;
            }

            // All fields matched, return true
            return true;
        }
    }

    // No match found in any exercise
    false
}