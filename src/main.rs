use sled;
mod utils;
mod models;

fn main() {
    // let db: sled::Db = sled::open("verse_db").unwrap();
    // db.insert(b"yo!", b"v1");
    // utils::display(&db.get(b"yo!").unwrap());
    // utils::count(&db);

    // let chapter = models::Chapter {
    //     name: "Al-Baqarah".into(),
    //     no: 1,
    // };
    let db: sled::Db = sled::open("chapter_db").unwrap();
    // utils::insert_chapter(&db, &chapter).unwrap();
    // utils::display(&db.get(b"yo!").unwrap());

    if db.is_empty().unwrap() {
        let chapters = utils::load_chapters_from_yaml("chapters.yaml");
    
        for chapter in chapters {
            utils::insert_chapter(&db, &chapter).unwrap();
        }
    }
    utils::count(&db);

    let chapter_name = utils::get_chapter_name(&db, 23).unwrap();
    println!("Chapter name: {:?}", chapter_name);
}