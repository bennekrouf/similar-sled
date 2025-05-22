mod files {
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

pub mod models;
pub mod api {
    pub mod check_discriminant;
    pub mod generate_exercise_endpoint;
    pub mod get_chapters;
    pub mod get_solutions;
    pub mod verse_by_chapter;
    pub mod verse_similar_by_chapter;
}
pub mod db {
    pub mod chapter {
        pub mod chapter_insert;
        pub mod chapter_name;
        pub mod chapters_init;
    }
    pub mod similar {
        pub mod similars_by_chapter;
        pub mod similars_by_key;
        pub mod similars_init;
        pub mod similars_insert;
    }
    pub mod exercise {
        pub mod check_discriminant;
        pub mod find_discriminant;
        pub mod get_solution;
    }
    pub mod verse {
        pub mod verse_insert;
        pub mod verses_by_chapter;
    }
    pub mod all_db;
}

mod utils {
    pub mod data_folder_path;
    pub mod extract_parts;
    pub mod sort;
    pub mod yml_path;
}

mod server;
mod validator;

#[rocket::main]
async fn main() {
    // Check deserialization of all files before starting the server
    if let Err(e) = validator::validate() {
        eprintln!("Error loading files: {}", e);
        std::process::exit(1);
    }

    // Launch the rocket server
    if let Err(e) = server::rocket().launch().await {
        eprintln!("Rocket launch failed: {}", e);
        std::process::exit(1);
    }
}

