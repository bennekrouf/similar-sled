use crate::models::UngroupedText;
use std::borrow::Cow;

// This function cleans the text by removing the "غغغ" pattern.
fn clean_text(input: Option<&str>) -> Cow<str> {
    let text: Cow<str> = match input {
        Some(t) => Cow::Borrowed(t),
        None => Cow::Borrowed(""),
    };
    Cow::Owned(text.replace("غغغ", ""))
}

pub fn extract_parts(text: Option<&str>) -> UngroupedText {
    match text {
        Some(t) => {
            if let Some(start) = t.find('[') {
                if let Some(end) = t.find(']') {
                    let pre = clean_text(Some(&t[0..start])).into_owned();
                    let discriminant = t[start..=end].to_string();  // Include the square brackets
                    let post = clean_text(Some(&t[end + 1..])).into_owned();

                    return UngroupedText {
                        pre: Some(pre),
                        discriminant: Some(discriminant),
                        post: Some(post),
                    };
                }
            }
            let cleaned_t = clean_text(Some(t)).into_owned();
            return UngroupedText {
                pre: Some(cleaned_t),
                discriminant: None,
                post: None,
            };
        }
        None => UngroupedText {
            pre: None,
            discriminant: None,
            post: None,
        },
    }
}
