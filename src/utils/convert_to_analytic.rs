use crate::models::SimilarOutputAdapted;
use std::collections::{HashSet, HashMap};
use crate::learning::models::analytic::Analytic;

pub fn convert_to_analytic(similar: &SimilarOutputAdapted) -> Vec<Analytic> {
    let mut all_ids = HashSet::new(); // Track all encountered IDs
    let mut kalima_ids = HashSet::new(); // Track unique kalima IDs
    let mut analytics = HashMap::new(); // Use a HashMap to store analytics

    // Helper function to add IDs and create Analytics
    fn add_id_and_analytic(
        all_ids: &mut HashSet<String>,
        kalima_ids: &mut HashSet<String>,
        analytics: &mut HashMap<String, f32>,
        id: String,
    ) {
        // Check if the ID is not already encountered
        if all_ids.insert(id.clone()) {
            // If it's a kalima ID, add it to the kalima_ids set
            if kalima_ids.contains(&id) {
                kalima_ids.insert(id.clone());
            }
            // Create an Analytic for this ID with progress 0.0
            analytics.insert(id, 0.0);
        }
    }

    // Iterate over verses, similars, and opposites
    for verse_list in [&similar.verses, &similar.similars, &similar.opposites].iter() {
        for verse in verse_list.iter() {
            let id = format!("{}-{}", verse.chapter_no, verse.verse_no);
            add_id_and_analytic(&mut all_ids, &mut kalima_ids, &mut analytics, id);
        }
    }

    // Iterate over unique kalima IDs
    for kalima_id in similar.kalima.split(',').map(|s| s.to_string()) {
        add_id_and_analytic(&mut all_ids, &mut kalima_ids, &mut analytics, kalima_id);
    }

    // Convert the HashMap into a Vec of Analytics
    let result: Vec<Analytic> = analytics
        .iter()
        .map(|(id, progress)| Analytic {
            id: id.to_string(),
            progress: *progress,
        })
        .collect();

    result
}
