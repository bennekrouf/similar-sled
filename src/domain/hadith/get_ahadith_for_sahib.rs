use crate::domain::hadith::models::{Hadith, Database}; // Import your existing structs here

pub fn get_ahadith_for_sahib(sahib_name: &str, dbs: &Database) -> Vec<Hadith> {
    let mousned_db = &dbs.mousned_db;
    let mut ahadith = Vec::new();

    if let Ok(Some(serialized_ahadith)) = mousned_db.get(sahib_name.as_bytes()) {
        if let Ok(decoded_ahadith) = serde_yaml::from_slice(&serialized_ahadith) {
            ahadith = decoded_ahadith;
        }
    }

    ahadith
}