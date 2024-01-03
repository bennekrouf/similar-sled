use std::fs::{self, File};
use std::io::Read;
use crate::utils::yml_path;
use crate::models::Similar;

use std::error::Error;
use std::path::Path;
use log::info;

pub fn load() -> Result<Vec<Similar>, Box<dyn Error>> {
    let data_folder_path = yml_path::get_data_folder_path();
    info!("Data folder path: {:?}", data_folder_path);

    let similars_yaml_path = data_folder_path.join("similars");

    let mut similars: Vec<Similar> = Vec::new();

    traverse_directory(&similars_yaml_path, &mut similars)?;

    Ok(similars)
}

fn traverse_directory(
    folder_path: &Path,
    similars: &mut Vec<Similar>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            traverse_directory(&path, similars)?;
        } else if let Some(extension) = path.extension() {
            if extension == "yml" {
                let path = entry.path().clone(); // Clone the path

                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                
                // Wrap the deserialization in a match block
                match serde_yaml::from_str::<Vec<Similar>>(&contents) {
                    Ok(file_similars) => {
                        for similar in &file_similars {
                            if similar.verses.is_empty() {
                                info!("Missing field in file: {:?}", &path);
                            }
                            // if similar.opposite_similars.as_ref().map_or(false, |v| !v.is_empty()) {
                            //     info!("opposite_similars field in file: {:?}", &path);
                            // }
                        }
                        similars.extend(file_similars);
                    }
                    Err(e) => {
                        // Return a new error that includes the path of the file that failed to deserialize
                        info!("Failed to deserialize file at {:?}: {}", path, e);
                        return Err(format!("Failed to deserialize file at {:?}: {}", path, e).into());
                    }
                }
            }
        }
    }

    Ok(())
}