use rocket::{get, State};
use rocket_contrib::json::Json;

use crate::domain::hadith::get_ahadith_for_sahib;
use crate::domain::hadith::models::Database;
use crate::domain::hadith::models::Hadith;

#[get("/hadith/<sahib_name>")]
pub fn get_ahadith_for_sahib(
    sahib_name: String,
    dbs: State<Database>,
) -> Json<Vec<Hadith>> {
    let ahadith = get_ahadith_for_sahib::get_ahadith_for_sahib(&sahib_name, &dbs);
    Json(ahadith) // Wrap the result in Rocket's Json
}