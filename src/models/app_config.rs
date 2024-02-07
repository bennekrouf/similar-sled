use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub macos_path: String,
    pub debian_path: String,
    pub port: u16,
}