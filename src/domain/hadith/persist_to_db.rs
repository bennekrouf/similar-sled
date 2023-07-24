use std::collections::HashMap;
use std::error::Error;
use crate::domain::hadith::models::{Database, Hadith}; // Import your existing structs here

pub fn persist_to_db(sahib: &str, ahadith: &Vec<Hadith>, dbs: &Database) -> Result<(), Box<dyn Error>> {
    let mousned_db = &dbs.mousned_db;
    let mut mousned_map: HashMap<String, Vec<Hadith>> = HashMap::new();

    // Custom cloning function for Hadith struct
    fn clone_hadith(hadith: &Hadith) -> Hadith {
        Hadith {
            matan: hadith.matan.clone(),
            riwayate: hadith.riwayate.clone(),
        }
    }

    // Manually clone each Hadith struct in the vector using the custom function
    let cloned_ahadith: Vec<Hadith> = ahadith.iter().map(clone_hadith).collect();
    mousned_map.insert(sahib.to_string(), cloned_ahadith);

    for (sahib, ahadith) in mousned_map {
        let serialized_sahib = serde_yaml::to_string(&sahib)?;
        let serialized_ahadith = serde_yaml::to_string(&ahadith)?;

        mousned_db.insert(serialized_sahib.as_bytes(), serialized_ahadith.as_bytes())?;
    }

    Ok(())
}
