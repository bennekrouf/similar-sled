use rand::seq::SliceRandom;

use crate::models::Statement;

pub fn extract_and_shuffle_options<T: PartialEq>(
    statements: &mut [Statement],  // change to mutable slice
    mut extractor: impl FnMut(&mut Statement) -> Option<T>,  // adjusted to accept &mut Statement
    exclude: &Option<T>
) -> Vec<T> {
    let mut options: Vec<T> = statements.iter_mut()  // iter_mut to get mutable references
        .filter_map(|statement| {
            let option = extractor(statement);
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
