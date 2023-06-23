#![feature(proc_macro_hygiene, decl_macro)]
// use sled::Db;
mod models;
mod init_db;
use api::verse::static_rocket_route_info_for_get_verse;
use api::similars::static_rocket_route_info_for_get_similars;
use rocket::{routes, Rocket};

// use rocket::fairing::AdHoc;
// use rocket_contrib::json::Json;
mod utils;
mod api {
    pub mod similars;
    pub mod verse;
}

fn rocket() -> Rocket {
    let data_folder_path = utils::get_data_folder_path();
    let database = init_db::init_all_db(&data_folder_path);

    rocket::ignite()
        .manage(database.clone())
        .mount("/", routes![get_verse, get_similars])
}

fn main() {
    rocket().launch();
}