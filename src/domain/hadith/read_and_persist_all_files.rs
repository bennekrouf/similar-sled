use std::collections::HashMap;
use std::fs;
use crate::domain::hadith::models::{Database, Hadith}; // Import your existing structs here
use crate::domain::hadith::read_mousned_from_file;
use crate::domain::hadith::persist_to_db;

pub fn read_and_persist_all_files(folder_path: &str, dbs: &Database) -> HashMap<String, Vec<Hadith>> {
    // let mousned_db = &dbs.mousned_db;

    let mut mousned_map: HashMap<String, Vec<Hadith>> = HashMap::new();

    // Iterate through all YAML files in the folder
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                    if let Some(filename) = path.to_str() {
                        match read_mousned_from_file::read_mousned_from_file(filename) {
                            Ok(mousned) => {
                                let ahadith: Vec<Hadith> = mousned.rouate.iter().flat_map(|bab| bab.ahadith.clone()).collect();
                                mousned_map.insert(mousned.sahib.clone(), ahadith.clone());

                                if let Err(err) = persist_to_db::persist_to_db(&mousned.sahib, &ahadith, &dbs) {
                                    eprintln!("Error persisting data for sahib '{}': {}", &mousned.sahib, err);
                                }
                            }
                            Err(err) => eprintln!("Error reading file '{}': {}", filename, err),
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading the folder '{}'", &folder_path);
    }

    mousned_map
}