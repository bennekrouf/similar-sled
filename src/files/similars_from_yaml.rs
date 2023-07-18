use std::fs::{self, File};
use std::io::Read;
use crate::utils::yml_path;
use crate::models::Similar;

use std::error::Error;
use std::path::Path;

pub fn load() -> Result<Vec<Similar>, Box<dyn Error>> {
    let data_folder_path = yml_path::get_data_folder_path();

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
                let mut file = File::open(path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let file_similars: Vec<Similar> = serde_yaml::from_str(&contents)?;
                similars.extend(file_similars);
            }
        }
    }

    Ok(())
}
