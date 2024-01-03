use std::fs::{self, File};
use std::io::Read;
use crate::utils::yml_path;
use crate::models::{Similar, YmlSimilar, VerseOutput};

use std::path::Path;
use log::info;
use crate::utils::extract_parts::extract_parts;

pub fn load() -> Result<Vec<Similar>, Box<dyn std::error::Error>> {
    let data_folder_path = yml_path::get_data_folder_path();
    info!("Data folder path: {:?}", data_folder_path);

    let similars_yaml_path = data_folder_path.join("similars");
    let yml_similars: Vec<YmlSimilar> = Vec::new();  // Initialize this if needed.

    // The traverse_directory function is now supposed to return Result<Vec<Similar>, Box<dyn Error>>
    let similars = traverse_directory(&similars_yaml_path, yml_similars)?;

    Ok(similars)
}

fn traverse_directory(folder_path: &Path, yml_similars: Vec<YmlSimilar>) -> Result<Vec<Similar>, Box<dyn std::error::Error>> {
    let mut output_similars: Vec<Similar> = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            output_similars.extend(traverse_directory(&path, yml_similars.clone())?);
        } else if let Some(extension) = path.extension() {
            if extension == "yml" {
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_yaml::from_str::<Vec<YmlSimilar>>(&contents) {
                    Ok(mut yml_similars_from_file) => {
                        for yml_similar in yml_similars_from_file.iter_mut() {
                            let mut verses: Vec<VerseOutput> = Vec::new();
                            for yml_verse in &mut yml_similar.verses {
                                let ungrouped_text = Some(extract_parts(Some(&yml_verse.text)));
                                verses.push(VerseOutput {
                                    chapter_no: yml_verse.chapter_no,
                                    verse_no: yml_verse.verse_no,
                                    sourate: None,
                                    ungrouped_text,
                                });
                            }

                            output_similars.push(Similar {
                                kalima: yml_similar.kalima.clone(),
                                opposites: yml_similar.opposites.clone(),
                                verses,
                            });
                        }
                    },
                    Err(e) => {
                        // Return a new error that includes the path of the file that failed to deserialize
                        info!("Failed to deserialize file at {:?}: {}", path, e);
                        return Err(format!("Failed to deserialize file at {:?}: {}", path, e).into());
                    }
                }
            }
        }
    }
    
    Ok(output_similars)
}
