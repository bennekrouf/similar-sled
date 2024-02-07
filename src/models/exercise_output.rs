use serde::{Serialize, Deserialize};
use super::statement::Statement;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseOutput {
    pub kalima: String,
    pub verses: Vec<Statement>,
}