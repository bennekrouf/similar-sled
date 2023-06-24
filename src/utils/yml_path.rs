use std::path::PathBuf;

pub fn get_data_folder_path() -> PathBuf {
    let mut path = PathBuf::new();
    if cfg!(target_os = "macos") {
        // Mac path
        path.push("/Volumes/Mayorana/code/similar/similars-sled/data");
    } else {
        // Debian path
        path.push("/home/similar/similar-sled/data");
    }
    path
}
