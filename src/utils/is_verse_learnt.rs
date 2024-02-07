use crate::learning::models::analytic::Analytic;

pub fn is_verse_learnt(verse_no: u32, analytics: &Vec<Analytic>) -> bool {
    // Display input parameters

    let result = analytics.iter().any(|analytic| {
        let analytic_verse_no = extract_verse_no(&analytic.id);
        // This println! is already in your function, displaying each analytic's verse number, the target verse number, and the progress
        if analytic_verse_no == verse_no {
            // println!("Checking verse_no = {:?}", verse_no);
            // println!("analytic.progress: {:?}", analytic.progress);
            analytic.progress > 99.0
        } else {
            false
        }
    });

    // Display the result of the function
    // println!("Verse {:?} learnt: {:?}", verse_no, result);
    
    result
}

fn extract_verse_no(id: &str) -> u32 {
    id.split(':').nth(1).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0)
}
