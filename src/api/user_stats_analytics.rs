use rocket::{post, State, serde::json::Json};
use crate::learning::{
    models::{
        user_stat::UserStat,
        analytic::Analytic,
        learning_config::LearningConfig
    },
    compute_user_stats_analytics::compute_user_stats_analytics
};
use crate::models::database::Database;
use crate::utils::parse_ranges::parse_ranges;
use crate::domain::verse::analytics_by_chapter::analytics_by_chapter;

#[post("/user-stats-analytics?<ranges>", format = "json", data = "<user_stats>")]
pub fn user_stats_analytics(
    dbs: &State<Database>,
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>, 
    ranges: Option<String>,
) -> Json<Vec<Analytic>> {
    // Compute user stats analytics
    let user_progress_analytics = compute_user_stats_analytics(&**config, &user_stats);

    // Parse ranges
    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r)).unwrap_or_else(Vec::new);
    
    // Initialize a map to store unique analytics by ID
    let mut analytics_map: std::collections::HashMap<String, Analytic> = user_progress_analytics
        .into_iter()
        .map(|analytic| (analytic.id.clone(), analytic))
        .collect();

    // for (id, analytic) in &analytics_map {
    //     println!("ID: {:?}, Analytic: {:?}", id, analytic);
    // }

    // Process each range and fetch analytics
    for range in parsed_ranges.iter() {
        let chapter_no_start = range.0;
        let chapter_no_end = range.1;
    
        for chapter_no in chapter_no_start..=chapter_no_end {
            if let Ok(chapter_analytics) = analytics_by_chapter(dbs, chapter_no as u8) {
                for analytic in chapter_analytics {
                    // println!("THE ID : {:?}", &analytic.id);
                    // Check if the analytic already exists in the map
                    if !analytics_map.contains_key(&analytic.id) {
                        // println!("ADDING : {:?}", &analytic.id);

                        // Insert analytics from chapters into the map if not already added by user progress
                        analytics_map.insert(analytic.id.clone(), analytic);
                    }
                }
            }
        }
    }

    // Convert the map back into a vector of analytics
    let mut result: Vec<Analytic> = analytics_map.into_values().collect();
    result.sort_by(|a, b| b.progress.partial_cmp(&a.progress).unwrap_or(std::cmp::Ordering::Equal));


    Json(result)
}
