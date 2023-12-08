use rocket::{get};
use rocket_contrib::json::JsonValue;
use std::fs;
use rocket_contrib::json;

#[get("/labels")]
pub fn get_labels() -> JsonValue {
    let file_path = "static/labels.json";
    match fs::read_to_string(file_path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(json) => json,
            Err(_) => json!({"error": "Failed to parse JSON."}),
        },
        Err(_) => json!({"error": "Failed to read file."}),
    }
}