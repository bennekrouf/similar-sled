#![feature(proc_macro_hygiene, decl_macro)]
// use sled::Db;
mod models;
mod init_db;
use api::verse::static_rocket_route_info_for_get_verse;
use api::similars::static_rocket_route_info_for_get_similars;
use rocket::{routes, Rocket};

// use rocket::fairing::AdHoc;
// use rocket_contrib::json::Json;
mod utils {
    pub mod chapter_name;
    pub mod count;
    pub mod data_folder_path;
    pub mod verses_by_chapter;
    pub mod verse_by_chapter_and_ayat;
}

mod api {
    pub mod similars;
    pub mod verse;
}

use crate::utils::data_folder_path;

fn rocket() -> Rocket {
    let data_folder_path = data_folder_path::get();
    let database = init_db::init_all_db(&data_folder_path);

    rocket::ignite()
        .manage(database.clone())
        .mount("/", routes![get_verse, get_similars])
}

fn main() {
    rocket().launch();
}