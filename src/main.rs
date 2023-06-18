
use sled;
mod utils;
mod models;
mod init_db;

fn main() {
    let db: sled::Db = sled::open("db/chapter_db").unwrap();
    // for i in 1..=114 {
    //     let chapter_name = utils::get_chapter_name(&db, i).unwrap();
    //     println!("{:?}", chapter_name.unwrap());
    // }
    init_db::init_chapters(&db);
    utils::count(&db);
}