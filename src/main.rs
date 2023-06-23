#![feature(proc_macro_hygiene, decl_macro)]
// use sled::Db;
mod models;
mod init_db;
use rocket::{routes, Rocket};
// use rocket::fairing::AdHoc;
// use rocket_contrib::json::Json;
mod api;
mod utils;

fn rocket() -> Rocket {
    let data_folder_path = utils::get_data_folder_path();
    let database = init_db::init_all_db(&data_folder_path);

    rocket::ignite()
        .manage(database.clone())
        // .attach(AdHoc::on_attach("Verse Database1", move |rocket| {
        //     let new_rocket = rocket.manage(database.clone());
        //     Ok(new_rocket)
        // }))
        .mount("/", routes![api::init, api::get_verse, api::get_similars])
}

fn main() {
    rocket().launch();
}