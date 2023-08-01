use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::models::Database;
use crate::domain::hadith::models::Hadith;
use crate::domain::hadith::get_ahadith_by_sahib;

#[get("/<sahib>")]
pub fn get_ahadith_by_sahib_route(
    dbs: State<Database>,
    sahib: String,
) -> Json<Vec<Hadith>> {
    let result = get_ahadith_by_sahib::get_ahadith_by_sahib(&dbs, sahib).unwrap_or_else(|_| vec![]);
    Json(result)
}

#[get("/counts")]
pub fn get_all_ahadith_counts_route(
    dbs: State<Database>,
) -> Json<Vec<(String, usize)>> {
    let result = get_ahadith_by_sahib::get_all_ahadith_counts(&dbs).unwrap_or_else(|_| vec![]);
    Json(result)
}
