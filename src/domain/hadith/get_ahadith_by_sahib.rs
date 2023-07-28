use bincode;
use rocket::State;
use crate::models::Database;
use crate::domain::hadith::models::Hadith;
use std::error::Error;

pub fn get_ahadith_by_sahib(dbs: &State<Database>, sahib: String) -> Result<Vec<Hadith>, Box<dyn Error>> {
    let mousned_db = &dbs.mousned_db;
    let serialized_key = sahib.as_bytes();

    match mousned_db.get(serialized_key)? {
        Some(ivec) => {
            let ahadith: Vec<Hadith> = bincode::deserialize(&*ivec)?;
            Ok(ahadith)
        },
        None => Ok(vec![]),
    }
}
