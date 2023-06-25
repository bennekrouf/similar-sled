#![feature(proc_macro_hygiene, decl_macro)]
mod models;
mod init {
    pub mod all_db;
    pub mod chapters;
    pub mod similars;
    pub mod chapter;
    pub mod verse;
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

use api::verse::static_rocket_route_info_for_get_verse;
use api::similars::static_rocket_route_info_for_get_similars;
use api::count::static_rocket_route_info_for_get;
use rocket::{routes, Rocket};

mod utils {
    pub mod chapter_name;
    pub mod count;
    pub mod data_folder_path;
    pub mod verses_by_chapter;
    pub mod verse_by_chapter_and_ayat;
    pub mod yml_path;
}

mod api {
    pub mod similars;
    pub mod verse;
    pub mod count;
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
        .mount("/", routes![get_verse, get_similars, get])
}

fn main() {
    rocket().launch();
}