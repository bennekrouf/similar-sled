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
use crate::domain::verse::analytics_skel_by_chapter_verse::analytics_skel_by_chapter_verse;

#[post("/chapter-stats-analytics?<ranges>", format = "json", data = "<user_stats>")]
pub fn chapter_stats_analytics(
    dbs: &State<Database>,
    ranges: Option<String>,
) -> Json<Vec<Analytic>> {
    // Parse ranges
    let parsed_ranges = ranges.as_ref().map(|r| parse_ranges(r)).unwrap_or_else(Vec::new);
    
    let mut chapter_progress_map: std::collections::HashMap<u32, Vec<f32>> = HashMap::new();

    // Process each range and fetch analytics
    for range in parsed_ranges.iter() {
        let chapter_no_start = range.0;
        let chapter_no_end = range.1;

        for chapter_no in chapter_no_start..=chapter_no_end {
            if let Ok(chapter_analytics) = analytics_skel_by_chapter_verse(dbs, chapter_no) {
                for analytic in chapter_analytics {
                    let chapter_no = extract_chapter_no(&analytic.id);
                    chapter_progress_map.entry(chapter_no).or_insert_with(Vec::new).push(analytic.progress);
                }
            }
        }
    }

    // Aggregate progress by chapter
    let aggregated_analytics: Vec<Analytic> = chapter_progress_map.iter().map(|(&chapter_no, progresses)| {
        let average_progress = if !progresses.is_empty() {
            progresses.iter().sum::<f32>() / progresses.len() as f32
        } else {
            0.0
        };

        Analytic {
            id: chapter_no.to_string(),
            progress: average_progress,
            category: Some("V".to_string()), // Adjust as needed
        }
    }).collect();

    Json(aggregated_analytics)
}

fn extract_chapter_no(id: &str) -> u32 {
    id.split(':').next().and_then(|c| c.parse::<u32>().ok()).unwrap_or(0)
}
