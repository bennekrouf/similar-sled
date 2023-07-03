use std::fs::{self, File};
use std::io::Read;
use crate::utils::yml_path;

use crate::models::Similar;

pub fn load() -> Result<Vec<Similar>, Box<dyn std::error::Error>> {
    let data_folder_path = yml_path::get_data_folder_path();

    let similars_yaml_path = data_folder_path.join("similars");
    let folder_path = similars_yaml_path.to_str().unwrap();

    let mut similars: Vec<Similar> = Vec::new();
    let folder = fs::read_dir(folder_path)?;

    for entry in folder {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "yml" {
                    let mut file = File::open(path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let file_similars: Vec<Similar> = serde_yaml::from_str(&contents)?;
                    similars.extend(file_similars);
                }
            }
        }
    }

    Ok(similars)
}