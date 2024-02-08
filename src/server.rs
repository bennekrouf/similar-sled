use std::env;
use log::LevelFilter;
use rocket::{
    http::Header, fairing::{Fairing, Info, Kind},
    config::Config, routes,
    Rocket, Build, Request, Response
};

use crate::utils::{data_folder_path, yml_path::{LEARNING, CONFIG}};
use crate::domain::all_db;

use crate::api::{
    ping::ping,
    verse_by_chapter::get_verse,
    get_chapters::get_chapters,
    get_labels::get_labels,
    generate_exercise_endpoint::generate_exercises_endpoint,
    verse_similar_by_chapter::get_verse_similar_by_chapter_route,
    verse_stats_analytics::verse_stats_analytics,
};

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

    // Load both configurations
    let app_config = CONFIG.lock().unwrap().clone();
    let learning_config = LEARNING.lock().unwrap().clone();

    let figment = Config::figment().merge(("port", app_config.port));

    // Start the Rocket application with the custom configuration
    rocket::build()
        .configure(figment)
        .attach(CORS)
        .manage(all_db.clone())
        .manage(app_config)
        .manage(learning_config)
        .mount("/", routes![
            get_verse,
            generate_exercises_endpoint,
            get_chapters,
            get_labels,
            ping,
            verse_stats_analytics,
            get_verse_similar_by_chapter_route,
        ])
}