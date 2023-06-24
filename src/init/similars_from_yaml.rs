use std::fs::File;
use std::io::Read;
use crate::models::Similar;

pub fn load(path: &str) -> Result<Vec<Similar>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let similars: Vec<Similar> = serde_yaml::from_str(&contents)?;
    Ok(similars)
}