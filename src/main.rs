#![feature(proc_macro_hygiene, decl_macro)]

// use sled::Db;
use std::path::Path;
mod utils;
mod models;
mod init_db;
use std::env;
use rocket::{routes, Rocket};
use rocket::fairing::AdHoc;
// use rocket_contrib::json::Json;
mod get;

fn rocket() -> Rocket {
    rocket::ignite()
    .attach(AdHoc::on_attach("Verse Database1", |rocket| {
       
         // Retrieve the command-line arguments
        let args: Vec<String> = env::args().collect();

        // Expect the second command-line argument to be the path to the "data" folder
        if args.len() < 2 {
            println!("Usage: ./my_program <data_folder_path>");
             std::process::exit(1);
        }

        let data_folder_path = &args[1];

        // Get the current directory of the executable
        let exe_dir = env::current_exe()
            .expect("Failed to get executable directory")
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();

        // Construct the absolute path to the data folder using the provided path
        let data_folder = Path::new(data_folder_path);
        let absolute_data_folder_path = exe_dir.join(data_folder);

        // Construct the absolute paths to the database files
        let chapter_db_path = absolute_data_folder_path.join("chapter_db");
        let verse_db_path = absolute_data_folder_path.join("verse_db");
        let similar_db_path = absolute_data_folder_path.join("similar_db");

        // Open the Sled databases using the adjusted file paths
        let chapter_db = sled::open(chapter_db_path).expect("Failed to open chapter database");
        let verse_db = sled::open(verse_db_path).expect("Failed to open verse database");
        let similar_db = sled::open(similar_db_path).expect("Failed to open similar database");

        let database = models::Database {
            chapter_db,
            verse_db,
            similar_db,
        };

        init_db::init_chapters(&database.chapter_db);
        init_db::init_similars(&database.similar_db, &database.verse_db);

        let new_rocket = rocket.manage(database);
        Ok(new_rocket)
    }))
    .mount("/", routes![get::get_verse, get::get_similar, get::get_similars])
}

fn main() {
    rocket().launch();
}