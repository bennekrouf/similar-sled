use std::error::Error;
use std::path::PathBuf;
use crate::domain::hadith::models::Database;
use crate::domain::hadith::read_and_persist_all_files;
use crate::utils::yml_path;

pub fn init(absolute_data_folder_path: &PathBuf) -> Result<Database, Box<dyn Error>> {
    // Construct the absolute paths to the database files
    let mousned_db_path = absolute_data_folder_path.join("mousned_db");
    let abwab_db_path = absolute_data_folder_path.join("abwab_db");

    // Open the Sled databases using the adjusted file paths
    let mousned_db = sled::open(mousned_db_path).expect("Failed to open mousned database");
    let abwab_db = sled::open(abwab_db_path).expect("Failed to open abawb database");

    let folder_path_str = yml_path::get_data_folder_path()
        .to_str()
        .expect("Invalid folder path")
        .to_owned(); // Convert to an owned String to prevent borrowing a temporary
    
    let database = Database {
        mousned_db,
        abwab_db,
    };

    // Call the read_and_persist_all_files function with the folder path as a string
    let mousned_map = read_and_persist_all_files::read_and_persist_all_files(&folder_path_str, &database);

    // chapters_init::init(&database);
    // similars_init::init(&database);

    Ok(database) // Return the Database wrapped in Ok()
}
