use std::str::FromStr;

pub fn parse_ranges(range_str: &str) -> Vec<(u8, u8)> {
    range_str.split(',')
        .filter_map(|range| {
            let bounds: Vec<&str> = range.split('-').collect();
            match bounds.as_slice() {
                [start, end] => {
                    let start = u8::from_str(start.trim()).ok()?;
                    let end = u8::from_str(end.trim()).ok()?;
                    Some((start, end))
                },
                [single] => {
                    let number = u8::from_str(single.trim()).ok()?;
                    Some((number, number)) // Treat single number as a range
                },
                _ => None,
            }
        })
        .collect()
}
