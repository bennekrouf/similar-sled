use crate::domain::coran::models::Similar;

pub fn list_similar_chapter_references(similar: &Similar) -> Vec<String> {
    similar
        .verses
        .iter()
        .map(|verse| format!("{}:{}", verse.chapter, verse.ayah))
        .collect()
}