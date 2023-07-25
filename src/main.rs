#![feature(proc_macro_hygiene, decl_macro)]
mod files {
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

mod domain {
    pub mod coran {
        pub mod models;
    }
    pub mod hadith {
        pub mod all_db;
        pub mod api {
            pub mod get_ahadith_for_sahib;
        }
        pub mod models;
        pub mod get_ahadith_for_sahib;
        pub mod persist_to_db;
        pub mod read_and_persist_all_files;
        pub mod read_mousned_from_file;
        pub mod create_reference_index;
        pub mod search_ahadith_by_reference;
    }
}

use api::verse_by_chapter::static_rocket_route_info_for_get_verse;
use api::similars_all::static_rocket_route_info_for_get_similars;
use api::similars_all::static_rocket_route_info_for_get_chapters;

use domain::hadith::api::get_ahadith_for_sahib::static_rocket_route_info_for_get_ahadith_for_sahib;

use api::verse_similar_by_chapter::static_rocket_route_info_for_get_verse_similar_by_chapter_route;
// use api::count::static_rocket_route_info_for_get;
use rocket::{routes, Rocket};
use std::env;
use log::LevelFilter;

// use crate::domain::coran::models::Database;
// use crate::domain::coran::models::Database;
// use crate::domain::hadith::models::Database as HadithDatabase;

mod utils {
    pub mod data_folder_path;
    pub mod yml_path;
    pub mod sort;
}

mod api {
    pub mod similars_all;
    pub mod verse_by_chapter;
    pub mod verse_similar_by_chapter;
}

mod db {
    pub mod chapter {
        pub mod chapter_name;
        pub mod chapter_insert;
        pub mod chapters_init;
    }
    pub mod similar {
        pub mod similars_insert;
        pub mod similars_init;
        pub mod similars_by_chapter;
        pub mod similars_by_key;
        pub mod similar_output_format;
        pub mod similars_all;
    }
    pub mod verse {
        pub mod verses_by_chapter;
        pub mod verse_by_chapter_and_ayat;
        pub mod verse_insert;
    }
    pub mod all_db;
}

use crate::utils::data_folder_path;
use crate::db::all_db;
use crate::domain::hadith::all_db as hadith_all_db;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

fn rocket() -> Rocket {
    let data_folder_path = data_folder_path::get();
    let coran_db = all_db::init(&data_folder_path);
    // Initialize the Hadith database with a default value (empty or placeholder Database)
    let hadith_db = match hadith_all_db::init(&data_folder_path) {
        Ok(db) => Some(db), // Successfully initialized database
        Err(err) => {
            eprintln!("Error initializing Hadith database: {}", err);
            // Return a placeholder value or None to represent the absence of a valid database.
            None // You can also use another value that makes sense for your application.
        }
    };
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to create CORS fairing.");

    rocket::ignite()
        .attach(cors)
        .manage(coran_db.clone())
        .manage(hadith_db.clone())
        .mount("/", routes![
            get_verse,
            get_similars,
            get_chapters,
            get_verse_similar_by_chapter_route,
            ])
        .mount("/hadith", routes![
            get_ahadith_for_sahib
            ])
}

fn main() {
    // Set the log level based on the RUST_LOG environment variable
   env::set_var("RUST_LOG", "info"); // Adjust log level as needed: error, warn, info, debug, trace
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None) // Disable timestamp
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();
    rocket().launch();
}