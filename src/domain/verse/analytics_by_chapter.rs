use rocket::State;
use crate::models::database::Database;
use sled::Result as SledResult;

use crate::learning::models::analytic::Analytic;

pub fn analytics_by_chapter(dbs: &State<Database>, chapter_no: u8) -> SledResult<Vec<Analytic>> {
    let verse_db = &dbs.verse_db;
    let prefix = format!("{}:", chapter_no);
    let mut analytics = Vec::new();

    for result in verse_db.scan_prefix(prefix) {
        if let Ok((key, _)) = result {
            let verse_key = String::from_utf8_lossy(&key).into_owned();
            analytics.push(Analytic {
                id: verse_key,
                progress: 0.0,
                category: Some("V".to_string()),
            });
        }
    }

    Ok(analytics)
}
