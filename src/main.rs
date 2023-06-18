
use sled;
mod utils;
mod models;
mod init_db;

fn main() {
    let chapter_db: sled::Db = sled::open("db/chapter_db").unwrap();
    let verse_db: sled::Db = sled::open("db/verse_db").unwrap();
    let similar_db: sled::Db = sled::open("db/similar_db").unwrap();
    // for i in 1..=114 {
    //     let chapter_name = utils::get_chapter_name(&db, i).unwrap();
    //     println!("{:?}", chapter_name.unwrap());
    // }
    init_db::init_chapters(&chapter_db);
    init_db::init_similars(&similar_db, &verse_db);
    
    let dbs = vec![&chapter_db, &verse_db, &similar_db];

    for db in dbs {
        utils::count(&db);
        let size = db.size_on_disk().expect("Failed to compute size");
        println!("Size of sled database: {} bytes", size);
    } 

}