#![feature(proc_macro_hygiene, decl_macro)]

use sled::Db;
mod utils;
mod models;
mod init_db;

use rocket::{get, routes, Rocket, State};
use rocket::fairing::AdHoc;
use rocket_contrib::json::Json;

struct Database {
    chapter_db: Db,
    verse_db: Db,
    similar_db: Db,
}

fn rocket() -> Rocket {
    rocket::ignite()
    .attach(AdHoc::on_attach("Verse Database1", |rocket| {
        let chapter_db = sled::open("db/chapter_db").expect("Failed to open chapter database");
        let verse_db = sled::open("db/verse_db").expect("Failed to open verse database");
        let similar_db = sled::open("db/similar_db").expect("Failed to open similar database");

        let database = Database {
            chapter_db,
            verse_db,
            similar_db,
        };

        init_db::init_chapters(&database.chapter_db);
        init_db::init_similars(&database.similar_db, &database.verse_db);

        let new_rocket = rocket.manage(database);
        Ok(new_rocket)
    }))
    .mount("/", routes![get_verse])
}

#[get("/verse/<chapter_no>")]
fn get_verse(chapter_no: u8, dbs: State<Database>) -> Json<serde_json::Value> {
    let chapter = utils::get_chapter_name(&dbs.chapter_db, chapter_no).unwrap();
    let verse = utils::get_verses_by_chapter(&dbs.verse_db, chapter_no).unwrap();

    // Create a JSON value using serde_json
    let json_value = serde_json::json!({
        "chapter": chapter,
        "verse": verse,
    });

    // Wrap the JSON value in a `Json` struct
    Json(json_value)
}

fn main() {
    // let chapter_db: sled::Db = sled::open("db/chapter_db").unwrap();
    // let verse_db: sled::Db = sled::open("db/verse_db").unwrap();
    // let similar_db: sled::Db = sled::open("db/similar_db").unwrap();
    // for i in 1..=114 {
    //     let chapter_name = utils::get_chapter_name(&db, i).unwrap();
    //     println!("{:?}", chapter_name.unwrap());
    // }
    
    // let dbs = vec![&chapter_db, &verse_db, &similar_db];
    // let dbs = vec![&similar_db];

    // for db in dbs {
    //     utils::count(&db);
    //     let size = db.size_on_disk().expect("Failed to compute size");
    //     println!("Size of sled database: {} bytes", size);
    // }

    // println!("{:?}", utils::get_chapter_name(&chapter_db, 2).unwrap());
    // println!("{:?}", utils::get_verses_by_chapter(&verse_db, 2).unwrap());

    rocket().launch();
}