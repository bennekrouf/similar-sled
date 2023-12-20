use std::fs;
use rocket::get;
use rocket::serde::json::Json;
use serde_json::{self, Value, json};

#[get("/labels")]
pub fn get_labels() -> Json<Value> {
    let file_path = "static/labels.json";
    match fs::read_to_string(file_path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(json) => Json(json),
            Err(_) => Json(json!({"error": "Failed to parse JSON."})),
        },
        Err(_) => Json(json!({"error": "Failed to read file."})),
    }
}
