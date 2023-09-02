use std::collections::HashSet;
use std::hash::Hash;

// Deduplicates a Vec<T> based on a specified field, identified by the key_selector closure.
// 
// Arguments:
// - `vec`: The original Vec<T> to deduplicate
// - `key_selector`: A closure that takes an &T and returns the field to deduplicate by
//
// Returns: A new Vec<T> containing unique items based on the specified field
//
// Example:
// ```
// let mut alternatives = vec![
//     Alternative { content: "A".to_string(), verse: Some("verse1".to_string()) },
//     Alternative { content: "A".to_string(), verse: Some("verse2".to_string()) },
//     Alternative { content: "B".to_string(), verse: Some("verse1".to_string()) },
// ];
// alternatives = deduplicate_by_field(alternatives, |alt| alt.verse.as_ref().map(|v| &v.content));
// ```
pub fn deduplicate_by_field<T, F, K>(vec: Vec<T>, mut key_selector: F) -> Vec<T>
where
    F: FnMut(&T) -> Option<K>,
    K: Eq + Hash,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    // Iterate through each item in the original Vec
    for item in vec {
        // Use the key_selector closure to determine the key for deduplication
        let key_option = key_selector(&item);
        
        // If the key has not been seen yet, add it to the result Vec and the HashSet
        if let Some(key) = key_option {
            if seen.insert(key) {
                result.push(item);
            }
        } else {
            // Handle the case where key_selector returns None (no key to deduplicate by)
            result.push(item);
        }
    }
    
    result
}
