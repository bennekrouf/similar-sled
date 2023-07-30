#![feature(proc_macro_hygiene, decl_macro)]
mod files {
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

pub mod models;
mod domain {
    pub mod coran {
        pub mod models;
        pub mod api {
                pub mod similars_all;
                pub mod verse_by_chapter;
                pub mod verse_similar_by_chapter;
            }
        }
    pub mod hadith {
        pub mod models;
        pub mod mousned_from_yaml;
        pub mod mousned_init;
        pub mod api;
        pub mod get_ahadith_by_sahib;
    }
}

use domain::hadith::api::static_rocket_route_info_for_get_ahadith_by_sahib_route;

use domain::coran::api::verse_by_chapter::static_rocket_route_info_for_get_verse;
use domain::coran::api::similars_all::static_rocket_route_info_for_get_similars;
use domain::coran::api::similars_all::static_rocket_route_info_for_get_chapters;

use domain::coran::api::verse_similar_by_chapter::static_rocket_route_info_for_get_verse_similar_by_chapter_route;
// use api::count::static_rocket_route_info_for_get;
use rocket::{routes, Rocket};
use std::env;
use log::LevelFilter;

mod utils {
    pub mod data_folder_path;
    pub mod yml_path;
    pub mod sort;
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

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

fn rocket() -> Rocket {
    let data_folder_path = data_folder_path::get();
    println!("Path to similarDB: {:?}", data_folder_path);

    let all_db = all_db::init(&data_folder_path);
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
        .manage(all_db.clone())
        .mount("/", routes![
            get_verse,
            get_similars,
            get_chapters,
            get_verse_similar_by_chapter_route,
            ])
        .mount("/ahadith", routes![
            get_ahadith_by_sahib_route,
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