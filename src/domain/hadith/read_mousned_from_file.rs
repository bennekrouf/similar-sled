use std::error::Error;
use std::fs;
use crate::domain::hadith::models::Mousned; // Import your existing structs here

pub fn read_mousned_from_file(filename: &str) -> Result<Mousned, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let mousned: Mousned = serde_yaml::from_str(&content)?;
    Ok(mousned)
}