use rand::Rng;

use crate::models::Statement;

// Instead of getting a mutable reference to the verse, just get its index
pub fn select_random_verse_index(statements: &Vec<Statement>) -> usize {
    rand::thread_rng().gen_range(0..statements.len())
}
