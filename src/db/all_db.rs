use std::path::PathBuf;

use crate::models::Database;
use crate::db::chapter::chapters_init;
use crate::db::similar::similars_init;
use crate::domain::hadith::mousned_init;

pub fn init(absolute_data_folder_path: &PathBuf) -> Database {
    // Construct the absolute paths to the database files
    let chapter_db_path = absolute_data_folder_path.join("chapter_db");
    let verse_db_path = absolute_data_folder_path.join("verse_db");
    let similar_db_path = absolute_data_folder_path.join("similar_db");
    let verse_similar_db_path = absolute_data_folder_path.join("verse_similar_db");
    // Hadith
    let mousned_db_path = absolute_data_folder_path.join("mousned_db");
    let abwab_db_path = absolute_data_folder_path.join("abwab_db");

    // Open the Sled databases using the adjusted file paths
    let chapter_db = sled::open(chapter_db_path).expect("Failed to open chapter database");
    let verse_db = sled::open(verse_db_path).expect("Failed to open verse database");
    let similar_db = sled::open(similar_db_path).expect("Failed to open similar database");
    let verse_similar_db = sled::open(verse_similar_db_path).expect("Failed to open similar database");
    // Hadith
    let mousned_db = sled::open(mousned_db_path).expect("Failed to open mousned database");
    let abwab_db = sled::open(abwab_db_path).expect("Failed to open abawb database");

    let database = Database {
        chapter_db,
        verse_db,
        similar_db,
        verse_similar_db,
        mousned_db,
        abwab_db,
    };

    chapters_init::init(&database);
    similars_init::init(&database);

    // Hadith
    mousned_init::init(&database);

    // Count the number of key/value pairs in each database and print
    let chapter_db_size = database.chapter_db.iter().count();
    println!("chapter_db contains {} key/value pairs", chapter_db_size);

    let verse_db_size = database.verse_db.iter().count();
    println!("verse_db contains {} key/value pairs", verse_db_size);

    let similar_db_size = database.similar_db.iter().count();
    println!("similar_db contains {} key/value pairs", similar_db_size);

    let verse_similar_db_size = database.verse_similar_db.iter().count();
    println!("verse_similar_db contains {} key/value pairs", verse_similar_db_size);

    let mousned_db_size = database.mousned_db.iter().count();
    println!("mousned_db contains {} key/value pairs", mousned_db_size);

    println!("Keys in mousned_db:");
    for result in database.mousned_db.iter() {
        let (key, _) = result.expect("Read error");
        if let Ok(key_str) = std::str::from_utf8(&key) {
            println!("{}", key_str);
        } else {
            println!("Non-string key found");
        }
    }
    
    database
}
