use sled::{Db, IVec};
use std::error::Error;
use crate::domain::hadith::models::{Hadith}; // Import your existing structs here
use serde_json::to_vec;

pub fn create_reference_index(db: &Db) -> Result<(), Box<dyn Error>> {
    let reference_index = db.open_tree(b"reference_index")?;

    for key in db.iter() {
        if let Ok((key, value)) = key {
            let sahib: String = serde_yaml::from_slice(&key)?;
            let ahadith: Vec<Hadith> = serde_yaml::from_slice(&value)?;

            for hadith in &ahadith {
                for riwaya in &hadith.riwayate {
                    for reference in &riwaya.references {
                        let key = format!("{}:{}", sahib, reference);
                        let serialized_hadith = to_vec(hadith)?;
                        reference_index.insert(key.as_bytes(), IVec::from(serialized_hadith))?;
                    }
                }
            }
        }
    }

    Ok(())
}
