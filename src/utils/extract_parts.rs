use crate::models::UngroupedText;

pub fn extract_parts(text: Option<&str>) -> UngroupedText {
    match text {
        Some(t) => {
            if let Some(start) = t.find('[') {
                if let Some(end) = t.find(']') {
                    let pre = &t[0..start];
                    let discriminant = &t[start + 1..end];
                    let post = &t[end + 1..];

                    return UngroupedText {
                        pre: Some(pre.to_string()),
                        discriminant: Some(discriminant.to_string()),
                        post: Some(post.to_string()),
                    };
                }
            }
            return UngroupedText {
                        pre: Some(t.to_string()),
                        discriminant: None,
                        post: None,
                    };
            // (Some(t.to_string()), None, None)
        }
        None => UngroupedText {
                        pre: None,
                        discriminant: None,
                        post: None,
                    },
    }
}
