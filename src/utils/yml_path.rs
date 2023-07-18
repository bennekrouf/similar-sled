use std::path::PathBuf;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use serde_yaml;

#[derive(Deserialize)]
struct Config {
    macos_path: String,
    debian_path: String,
}

pub fn get_data_folder_path() -> PathBuf {
    let mut path = PathBuf::new();

    // Read the config file
    let mut config_file = File::open("config.yml").expect("Failed to open config file");
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).expect("Failed to read config file");

    // Deserialize the YAML config
    let config: Config = serde_yaml::from_str(&config_content).expect("Failed to parse config file");

    if cfg!(target_os = "macos") {
        path.push(config.macos_path);
    } else {
        path.push(config.debian_path);
    }

    path
}