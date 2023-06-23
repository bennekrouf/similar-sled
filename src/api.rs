use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{Similar, Verse, Database};
use crate::init_db::{init_chapters, init_similars};
use crate::utils;

#[get("/init")]
pub fn init(database: State<Database>){

    database.chapter_db.flush().unwrap();
    database.verse_db.flush().unwrap();
    database.similar_db.flush().unwrap();

    init_chapters(&database.chapter_db);
    init_similars(&database.similar_db, &database.verse_db);

    // let data_folder_path = utils::get_data_folder_path();
    // print!("{:?}", data_folder_path);
    // init_db::init_all_db(&data_folder_path);
}

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<Similar>> {
    let similars: Vec<Similar> = dbs
        .similar_db
        .iter()
        .map(|result| {
            let (key, value) = result.expect("Failed to retrieve similar");
            let key_string = String::from_utf8_lossy(&key).into_owned();
            println!("Similar: {}", key_string);
            let references: Vec<(u32, u32)> =
                bincode::deserialize(&value).expect("Failed to deserialize references");

            let updated_references: Vec<Verse> = references
                .iter()
                .map(|&reference| {
                    let ayat = reference.1;
                    let chapter_no = reference.0;
                    // let chapter_name = utils::get_chapter_name(&dbs.chapter_db, chapter_no as u8)
                    //     .unwrap_or_else(|_| Some(String::from("Default Chapter")))
                    //     .unwrap();

                    let verse_text = match utils::get_verse_by_chapter_and_ayat(
                        &dbs.verse_db,
                        reference.0,
                        ayat,
                    ) {
                        Ok(Some(text)) => text,
                        Ok(None) => String::from("Verse not found"),
                        Err(_) => String::from("Error retrieving verse"),
                    };

                    Verse {
                        text: verse_text,
                        ayat,
                        chapter: chapter_no,
                    }
                })
                .collect();

            Similar {
                text: key_string,
                verses: updated_references,
            }
        })
        .map(|x| {
            println!("{:?}", x.verses[0]);
            x
        })
        .collect();

    Json(similars)
}


#[get("/verse/<chapter_no>")]
pub fn get_verse(chapter_no: u8, dbs: State<Database>) -> Json<serde_json::Value> {
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