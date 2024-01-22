pub fn is_chapter_in_range(chapter_no: &u32, chapter_range: &Option<Vec<(u8, u8)>>) -> bool {
    match chapter_range {
        Some(ranges) => ranges.iter().any(|&(start, end)| *chapter_no >= start as u32 && *chapter_no <= end as u32),
        None => true, // If no range is specified, all chapters are considered
    }
}