#![feature(proc_macro_hygiene, decl_macro)]
mod models;
mod init {
    pub mod all_db;
    pub mod chapters;
}

mod files {
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

use api::verse_by_chapter::static_rocket_route_info_for_get_verse;
use api::similars_all::static_rocket_route_info_for_get_similars;
// use api::count::static_rocket_route_info_for_get;
use rocket::{routes, Rocket};

mod utils {
    pub mod data_folder_path;
    pub mod yml_path;
    pub mod sort;
}

mod api {
    pub mod similars_all;
    pub mod verse_by_chapter;
    pub mod count;
}

mod db {
    pub mod chapter_name;
    pub mod count;
    pub mod verses_by_chapter;
    // pub mod all_similars;
    pub mod verse_by_chapter_and_ayat;
    pub mod insert_chapter;
    pub mod insert_similars;
    pub mod init_similars;
    pub mod insert_verse;
}

use crate::utils::data_folder_path;
use crate::init::all_db;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

fn rocket() -> Rocket {
    let data_folder_path = data_folder_path::get();
    let database = all_db::init(&data_folder_path);
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
        .manage(database.clone())
        .mount("/", routes![
            get_verse,
            get_similars,
            // get,
            ])
}

fn main() {
    rocket().launch();
}