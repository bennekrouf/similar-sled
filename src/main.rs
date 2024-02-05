mod files;
mod learning;
pub mod models;
mod api;
mod domain;
mod utils;
mod validator;
mod server;
mod xhr_guard;

#[tokio::main]
async fn main() {
    // Check deserialization of all files before starting the server
    if let Err(e) = validator::validate() {
        eprintln!("Similar error loading validator files: {}", e);
        std::process::exit(1);  // Exit the program with a non-zero status code
    }

    server::start_server().await;
}