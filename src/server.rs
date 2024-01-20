use rocket::{routes};
use rocket::config::Config;
// use rocket::http::Method;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Rocket, Build, Request, Response};
use crate::api::ping::ping;

use std::env;
use log::LevelFilter;

use crate::utils::data_folder_path;
use crate::domain::all_db;

use crate::api::verse_by_chapter::get_verse;
use crate::api::get_chapters::get_chapters;
use crate::api::get_labels::get_labels;
use crate::api::generate_exercise_endpoint::generate_exercise_list_endpoint;

// use crate::api::verse_similar_by_chapter::static_rocket_route_info_for_get_verse_similar_by_chapter_route;
use crate::api::verse_similar_by_chapter::get_verse_similar_by_chapter_route;
use crate::utils::yml_path::load_config;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub async fn start_server() {
    // Set the log level based on the RUST_LOG environment variable
    env::set_var("RUST_LOG", "info"); // Adjust log level as needed: error, warn, info, debug, trace
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None) // Disable timestamp
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    rocket().launch().await.expect("server failed to launch");
}

fn rocket() -> Rocket<Build> {
    let data_folder_path = data_folder_path::get();
    println!("Path to similarDB: {:?}", data_folder_path);

    let all_db = all_db::init(&data_folder_path);

    // Get the APP_ENV environment variable
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    // Load the config based on APP_ENV
    let config_data = load_config(&app_env);

    // let mut config = Config::figment().clone();
    // config.set_port(config_data.port);

    let figment = Config::figment()
        .merge(("port", config_data.port));

    // Start the Rocket application with the custom configuration
    rocket::build()
        .configure(figment)
        .attach(CORS)
        .manage(all_db.clone())
        .mount("/", routes![
            get_verse,
            generate_exercise_list_endpoint,
            get_chapters,
            get_labels,
            ping,
            get_verse_similar_by_chapter_route,
        ])
}