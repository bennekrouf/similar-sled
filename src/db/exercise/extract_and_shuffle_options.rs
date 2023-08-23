use rand::seq::SliceRandom;
use rand::Rng;

use crate::models::Statement;

pub fn extract_and_shuffle_options<T: PartialEq>(
    statements: &[Statement],
    mut extractor: impl FnMut(&Statement) -> Option<T>, // change to FnMut here
    exclude: &Option<T>
) -> Vec<T> {
    let mut options: Vec<T> = statements.iter()
        .filter_map(|statement| {
            let option = extractor(statement);  // since we have changed it to FnMut, it's mutable now
            if let Some(exclude_value) = exclude {
                if option.as_ref() == Some(&exclude_value) {
                    return None;
                }
            }
            option
        })
        .collect();

    options.shuffle(&mut rand::thread_rng());
    options
}

// Instead of getting a mutable reference to the verse, just get its index
fn select_random_verse_index(statements: &Vec<Statement>) -> usize {
    rand::thread_rng().gen_range(0..statements.len())
}
