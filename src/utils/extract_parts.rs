pub fn extract_parts(text: &str) -> (Option<String>, Option<String>, Option<String>) {
    if let Some(start) = text.find('[') {
        if let Some(end) = text.find(']') {
            let pre = &text[0..start];
            let discriminant = &text[start+1..end];
            let post = &text[end+1..];

            return (Some(pre.to_string()), Some(discriminant.to_string()), Some(post.to_string()));
        }
    }
    (Some(text.to_string()), None, None)
}