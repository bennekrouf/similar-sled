use std::env;
use std::fs::{self, DirEntry};
use std::io::Result;
use std::path::Path;
use regex::Regex;

// Function to process each file
fn process_file(entry: &DirEntry) -> Result<()> {
    let path = entry.path();
    let filename = path.file_name().unwrap().to_str().unwrap();

    // Process only .yml files and skip files that already have the "-" prefix
    if !filename.ends_with(".yml") || filename.starts_with("-") {
        return Ok(());
    }

    let new_filename = format!("-{}", filename); // Prepend '-' to the filename
    let new_path = path.with_file_name(new_filename);

    let content = fs::read_to_string(&path)?;
    let lrm = '\u{200E}';
    let rlm = '\u{200F}';
    let re = Regex::new(r"'([\u0600-\u06FF]+[^']*)'").unwrap();
    let updated_content = re.replace_all(&content, |caps: &regex::Captures| {
        format!("'{}[{}{}{}]{}'", lrm, rlm, &caps[1], lrm, rlm)
    });

    fs::write(new_path, updated_content.as_ref())?;
    Ok(())
}

// Function to process all files in a directory recursively
fn process_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_directory(path)?; // Recursive call for directories
        } else if path.is_file() {
            process_file(&entry)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // Specify the path to the data folder relative to the current working directory
    let data_dir = env::current_dir()?.join("../../data");

    process_directory(data_dir)
}
