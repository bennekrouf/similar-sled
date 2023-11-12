#![feature(proc_macro_hygiene, decl_macro)]
mod files {
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

pub mod models;
pub mod api {
        pub mod check_discriminant;
        pub mod check_chapter;
        pub mod generate_exercise_endpoint;
        pub mod get_chapters;
        pub mod get_solutions;
        pub mod verse_by_chapter;
        pub mod verse_similar_by_chapter;
}
pub mod domain {
    pub mod chapter {
        pub mod chapter;
        pub mod chapters_init;
    }
    pub mod similar {
        pub mod similars_init;
        pub mod similars_by_chapter;
        pub mod similars_by_key;
        pub mod similars_by_key_count;
        pub mod sourate_from_verse;
    }
    pub mod exercise {
        pub mod get_exercises;
        pub mod generate;
        pub mod check_discriminant;
        pub mod exercise_type_impl;
        pub mod check_chapter;
        pub mod get_solution;
        pub mod extract_and_shuffle_options;
        pub mod select_random_verse_index;
        pub mod sort_exercises;
        pub mod exercises_for_similar;
    }
    pub mod verse {
        pub mod verses_by_chapter;
        pub mod count_verses_in_chapter;
    }
    pub mod all_db;
}

mod utils {
    pub mod data_folder_path;
    pub mod yml_path;
    pub mod sort;
    pub mod extract_parts;
    pub mod insert_in_sled;
    pub mod parse_ranges;
}

mod validator;
mod server;
mod xhr_guard;
fn main() {
    // Check deserialization of all files before starting the server
    if let Err(e) = validator::validate() {
        eprintln!("Similar error loading validator files: {}", e);
        std::process::exit(1);  // Exit the program with a non-zero status code
    }

    server::start_server();
}