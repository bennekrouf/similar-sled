use std::str::FromStr;

pub fn parse_ranges(range_str: &str) -> Vec<(u8, u8)> {
    range_str.split(',')
        .filter_map(|range| {
            let bounds: Vec<&str> = range.split('-').collect();
            if bounds.len() == 2 {
                let start = u8::from_str(bounds[0]).ok()?;
                let end = u8::from_str(bounds[1]).ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}