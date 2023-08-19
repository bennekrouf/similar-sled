use rocket::{routes, Rocket};
use rocket::config::{Config, Environment};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use std::env;
use log::LevelFilter;

use crate::utils::data_folder_path;
use crate::db::all_db;

use crate::api::verse_by_chapter::static_rocket_route_info_for_get_verse;
use crate::api::get_chapters::static_rocket_route_info_for_get_chapters;
use crate::api::get_solutions::static_rocket_route_info_for_get_solutions;
use crate::api::check_discriminant::static_rocket_route_info_for_check_discriminant;
use crate::api::generate_exercise_endpoint::static_rocket_route_info_for_generate_exercise_endpoint;

use crate::api::verse_similar_by_chapter::static_rocket_route_info_for_get_verse_similar_by_chapter_route;
use crate::utils::yml_path::load_config;

pub fn start_server() {
    // Set the log level based on the RUST_LOG environment variable
    env::set_var("RUST_LOG", "info"); // Adjust log level as needed: error, warn, info, debug, trace
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None) // Disable timestamp
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    rocket().launch();
}

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

    // Get the APP_ENV environment variable
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    // Load the config based on APP_ENV
    let config_data = load_config(&app_env);

    // Use the port from the config_data
    let port = config_data.port;
    let rocket_env = match app_env.as_str() {
        "local" => Environment::Development,
        "staging" => Environment::Staging,
        "production" => Environment::Production,
        _ => Environment::Development, // default to Development if none of the above
    };

    let config = Config::build(rocket_env)
        .port(port)
        .finalize()
        .unwrap();

    // Start the Rocket application with the custom configuration
    rocket::custom(config)
        .attach(cors)
        .manage(all_db.clone())
        .mount("/", routes![
            get_verse,
            get_solutions,
            check_discriminant,
            generate_exercise_endpoint,
            get_chapters,
            get_verse_similar_by_chapter_route,
        ])
}