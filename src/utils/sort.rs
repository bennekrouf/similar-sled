use crate::models::SimilarOutput;

pub fn sort_similars(similars: &mut [SimilarOutput]) {
    similars.sort_by(|a, b| {
        let verse_len_cmp = a.verses.len().cmp(&b.verses.len());
        if verse_len_cmp == std::cmp::Ordering::Equal {
            let a_chapter_sum: u32 = a.verses.iter().map(|verse| verse.verse.chapter).sum();
            let b_chapter_sum: u32 = b.verses.iter().map(|verse| verse.verse.chapter).sum();
            a_chapter_sum.cmp(&b_chapter_sum)
        } else {
            verse_len_cmp
        }
    });
}