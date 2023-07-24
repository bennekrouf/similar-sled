use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use sled::{Config, Db, IVec};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use hadith::models::{Mousned, Hadith, Riwaya}; // Import your existing structs here

pub fn create_reference_index(db: &Db) -> Result<(), Box<dyn Error>> {
    let reference_index = db.open_tree(b"reference_index")?;

    for key in db.iter() {
        if let Ok((key, value)) = key {
            let sahib: String = serde_yaml::from_slice(&key)?;
            let ahadith: Vec<Hadith> = serde_yaml::from_slice(&value)?;

            for hadith in &ahadith {
                for riwaya in &hadith.riwayate {
                    if let Some(references) = &riwaya.reference {
                        for reference in references {
                            let key = format!("{}:{}", sahib, reference);
                            reference_index.insert(key.as_bytes(), IVec::from(hadith.clone()))?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}