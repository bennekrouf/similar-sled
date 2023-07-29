use std::path::{Path, PathBuf};
use users::get_current_username;
use dirs::home_dir;

pub fn get() -> PathBuf {
    let username = get_current_username()
        .expect("Failed to retrieve current username")
        .into_string()
        .expect("Failed to convert username to string");

    if let Some(mut home_path) = home_dir() {
        home_path.push("similarDB");
        return home_path;
    }
    Path::new("/tmp").join(&username).join("similarDB")
}