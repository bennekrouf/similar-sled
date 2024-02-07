use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ExerciseType {
    FindDiscriminant,
    FindSourate,
    C,
}