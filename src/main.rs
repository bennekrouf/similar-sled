#![feature(proc_macro_hygiene, decl_macro)]
mod files {
    pub mod chapters_from_yaml;
    pub mod similars_from_yaml;
}

pub mod models;
mod domain {
    pub mod coran {
        pub mod models;
        pub mod api {
                pub mod similars_all;
                pub mod verse_by_chapter;
                pub mod verse_similar_by_chapter;
            }
        }
    pub mod hadith {
        pub mod models;
        pub mod mousned_from_yaml;
        pub mod mousned_init;
        pub mod api;
        pub mod get_ahadith_by_sahib;
    }
}

mod utils {
    pub mod data_folder_path;
    pub mod yml_path;
    pub mod sort;
}

mod db {
    pub mod chapter {
        pub mod chapter_name;
        pub mod chapter_insert;
        pub mod chapters_init;
    }
    pub mod similar {
        pub mod similars_insert;
        pub mod similars_init;
        pub mod similars_by_chapter;
        pub mod similars_by_key;
        // pub mod similar_output_format;
        // pub mod similars_all;
    }
    pub mod verse {
        pub mod verses_by_chapter;
        // pub mod verse_by_chapter_and_ayat;
        pub mod verse_insert;
    }
    pub mod all_db;
}

mod validator;
mod server;

fn main() {
    // Check deserialization of all files before starting the server
    if let Err(e) = validator::validate() {
        eprintln!("Error loading files: {}", e);
        std::process::exit(1);  // Exit the program with a non-zero status code
    }

    server::start_server();
}