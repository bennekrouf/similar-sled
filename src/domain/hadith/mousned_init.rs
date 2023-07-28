use crate::domain::hadith::mousned_from_yaml::load;
use crate::models::Database;
use crate::domain::hadith::models::Mousned;
use bincode;

use std::path::Path;
use sled::IVec;
use sled::Result as SledResult;

fn persist_data(mousned_vec: &Vec<Mousned>, db: &Database) -> SledResult<()> {
    for mousned in mousned_vec {
        let key = mousned.sahib.clone().unwrap_or_default(); // Use a default string if sahib is None
        let value = bincode::serialize(&mousned.ahadith).expect("Failed to serialize mousned");
        db.mousned_db.insert(key.as_bytes(), IVec::from(value))?;
    }
    Ok(())
}

pub fn init(dbs: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let mousned_vec = load(Path::new("./data/hadith"), None)?;
    persist_data(&mousned_vec, &dbs)?;
    Ok(())
}