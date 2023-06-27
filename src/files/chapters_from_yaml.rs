use std::fs::File;
use std::io::Read;
use crate::models::Chapter;

pub fn load(file_path: &str) -> Vec<Chapter> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    serde_yaml::from_str(&contents).expect("Unable to parse YAML data")
}
