use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use sled::{Config, Db};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::domain::hadith::models::{Mousned, Hadith}; // Import your existing structs here

pub fn search_ahadith_by_reference(search_item: &str, db: &Db) -> Result<Vec<Hadith>, Box<dyn Error>> {
    let reference_index = db.open_tree(b"reference_index")?;
    let mut matching_ahadith = Vec::new();

    if let Some(mut cursor) = reference_index.scan_prefix(search_item.as_bytes()) {
        while let Some((key, value)) = cursor.next() {
            let sahib_and_reference = std::str::from_utf8(&key)?;
            let (_, reference) = sahib_and_reference.split_at(sahib_and_reference.find(':').unwrap());
            if reference.trim() == search_item {
                let ahadith: Vec<Hadith> = serde_yaml::from_slice(&value)?;
                matching_ahadith.extend(ahadith);
            }
        }
    }

    Ok(matching_ahadith)
}