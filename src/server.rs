use rocket::http::Method;
use rocket::Config;
use rocket::{routes, Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use log::LevelFilter;
use std::env;

use crate::db::all_db;
use crate::utils::data_folder_path;

use crate::api::check_discriminant::check_discriminant_endpoint;
use crate::api::generate_exercise_endpoint::generate_exercise_endpoint;
use crate::api::get_chapters::get_chapters;
use crate::api::get_solutions::get_solutions;
use crate::api::verse_by_chapter::get_verse;
use crate::api::verse_similar_by_chapter::get_verse_similar_by_chapter_route;
use crate::utils::yml_path::load_config;

pub fn rocket() -> Rocket<Build> {
    // Set the log level based on the RUST_LOG environment variable
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

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

    // Get the APP_ENV environment variable
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    // Load the config based on APP_ENV
    let config_data = load_config(&app_env);

    // Use the port from the config_data
    let port = config_data.port;

    // Create a custom configuration
    let config = Config {
        port,
        address: "0.0.0.0".parse().expect("Valid address"),
        ..Config::default()
    };

    rocket::custom(config).attach(cors).manage(all_db).mount(
        "/",
        routes![
            get_verse,
            get_solutions,
            check_discriminant_endpoint,
            generate_exercise_endpoint,
            get_chapters,
            get_verse_similar_by_chapter_route,
        ],
    )
}
