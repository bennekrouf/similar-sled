use rocket::{post, State};
use rocket::serde::json::Json;

use crate::learning::models::{
    user_stat::UserStat,
    analytic::Analytic,
    learning_config::LearningConfig
};
use crate::learning::compute_user_stats_analytics::compute_user_stats_analytics;
use crate::models::Database;
use crate::utils::parse_ranges::parse_ranges;
use crate::domain::similar::similars_by_chapter::similars_by_chapter;
use crate::utils::convert_to_analytic::convert_to_analytic;

#[post("/user-stats-analytics?<ranges>", format = "json", data = "<user_stats>")]
pub fn user_stats_analytics(
    dbs: &State<Database>,
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>, 
    ranges: Option<String>,
) 
-> Json<Vec<Analytic>> {
    let item_progress = compute_user_stats_analytics(&**config, &user_stats);

    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r));
    
    // Initialize an empty Vec to collect the results
    let mut result: Vec<Analytic> = Vec::new();

    // Use map to iterate over parsed_ranges and call similars_by_chapter for each item
    if let Some(parsed_ranges) = &parsed_ranges {
        for range in parsed_ranges {
            // Assuming range is a tuple (u8, u8)
            let chapter_no_start = range.0 as u32;
            let chapter_no_end = range.1 as u32;

            // Call similars_by_chapter for each chapter in the range
            for chapter_no in chapter_no_start..=chapter_no_end {
                let similars_adapted = similars_by_chapter(&dbs, chapter_no, &Some(parsed_ranges.to_vec()));

                // Iterate over each SimilarOutputAdapted in the Vec and convert it to Analytic
                for similar_adapted in &similars_adapted {
                    // Add some debug print statements
                    println!("Similar Adapted: {:?}", similar_adapted);

                    let analytics = convert_to_analytic(similar_adapted);

                    // Add some debug print statements
                    println!("Analytics: {:?}", analytics);

                    // Add the analytic to the result
                    result.extend(analytics);
                }
            }
        }
    }
    
    // Add some debug print statements
    println!("Result: {:?}", result);

    Json(result)
}
