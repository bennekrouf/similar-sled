use rocket::{get, State};
use rocket_contrib::json::Json;
use crate::models::{SimilarOutput, Verse, VerseOutput, Database};
use crate::db::verse_by_chapter_and_ayat;
use crate::db::chapter_name;
use crate::utils::sort;

#[get("/similars")]
pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
    let mut similars = dbs
        .similar_db
        .iter()
        .map(|result| {
            let (similar_key, similar_value) = result.expect("Failed to retrieve similar");
            let kalima = String::from_utf8_lossy(&similar_key).into_owned();
            let references: Vec<String> =
                bincode::deserialize(&similar_value).expect("Failed to deserialize references");

            let verses: Vec<VerseOutput> = references
                .iter()
                .map(|reference| {

                    let split: Vec<&str> = reference.split(":").collect();
                    let chapter: u32 = split[0].parse().expect("Not a valid u32");
                    let ayat: u32 = split[1].parse().expect("Not a valid u32");
                    
                    let chapter_name_result = chapter_name::get(&dbs.chapter_db, chapter as u8);
                    let chapter_name = match chapter_name_result {
                        Ok(Some(name)) => name,
                        Ok(None) | Err(_) => String::from("Default Chapter"),
                    };

                    println!("Search verse by : {} {}", chapter, ayat);

                    let text = match verse_by_chapter_and_ayat::get(
                        &dbs.verse_db,
                        chapter,
                        ayat,
                    ) {
                        Ok(Some(verse_text)) => verse_text,
                        Ok(None) => String::from("Verse not found 1"),
                        Err(_) => String::from("Error retrieving verse 1"),
                    };

                    VerseOutput {
                        sourate: chapter_name,
                        verse: Verse {
                            text,
                            ayat,
                            chapter,
                        },
                    }
                })
                .collect();

            SimilarOutput {
                kalima,
                verses,
            }
        })
        .collect::<Vec<SimilarOutput>>();

    sort::sort_similars(&mut similars);
    Json(similars)
}


// #[get("/similars")]
// pub fn get_similars(dbs: State<Database>) -> Json<Vec<SimilarOutput>> {
//     let similars: Vec<SimilarOutput> = dbs
//         .similar_db
//         .iter()
//         .map(|result| {
//             let (key, value) = result.expect("Failed to retrieve similar");
//             let kalima = String::from_utf8_lossy(&key).into_owned();
//             let references: Vec<(u32, u32)> =
//                 bincode::deserialize(&value).expect("Failed to deserialize references");

//             let verses: Vec<VerseOutput> = references
//                 .iter()
//                 .map(|&reference| {
//                     let chapter = reference.0;
//                     let ayat = reference.1;

//                     let chapter_name_result = chapter_name::get(&dbs.chapter_db, chapter as u8);

//                     let chapter_name = match chapter_name_result {
//                         Ok(Some(name)) => name,
//                         Ok(None) | Err(_) => String::from("Default Chapter"),
//                     };


//                     // let chapter_name = chapter_name::get(&dbs.chapter_db, chapter as u8)
//                     //     .unwrap_or_else(|_| Some(String::from("Default Chapter")))
//                     //     .unwrap();

//                     let text = match verse_by_chapter_and_ayat::get(
//                         &dbs.verse_db,
//                         chapter,
//                         ayat,
//                     ) {
//                         Ok(Some(verse_text)) => verse_text,
//                         Ok(None) => String::from("Verse not found 0"),
//                         Err(_) => String::from("Error retrieving verse"),
//                     };

//                     let verse =  Verse {
//                         text,
//                         ayat,
//                         chapter,
//                     };

//                     VerseOutput {
//                         verse,
//                         sourate: chapter_name, 
//                     }
//                 })
//                 .collect();

//             SimilarOutput {
//                 kalima,
//                 verses,
//             }
//         })
//         .collect();
//     Json(similars)
// }