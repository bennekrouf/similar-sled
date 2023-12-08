use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct RangeEntry {
    pub name: String,
    pub start: u8,
    pub end: u8,
    pub section: String,
}

pub fn read_default_range_from_json() -> Option<(u8, u8)> {
    let file_path = "static/labels.json";
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            match serde_json::from_str::<Vec<RangeEntry>>(&contents) {
                Ok(entries) => {
                    if let Some(first_entry) = entries.first() {
                        Some((first_entry.start, first_entry.end))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}
