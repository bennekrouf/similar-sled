pub fn count(db: &sled::Db) {
    let count = db.iter().keys().count();
    println!("There are {} keys in the database", count);
}