use serde::{Serialize, Deserialize};
use super::exercise_type::ExerciseType;
use super::statement::Statement;
use super::alternative::Alternative;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Exercise {
    pub exercise_type: ExerciseType,
    pub statement: Statement,
    pub alternatives: Vec<Alternative>,
}
