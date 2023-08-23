use crate::models::{ExerciseType, Statement, Database, Alternative};
use crate::db::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::db::exercise::select_random_verse_index::select_random_verse_index;
use crate::db::exercise::get_solution::get_solution;

pub fn generate(dbs: &Database, kalima: String) -> Option<(Statement, Vec<Alternative>, ExerciseType)> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }

    let exercise = &mut exercises[0];

    // Step 1: Get the index of the random verse
    let selected_verse_index = select_random_verse_index(&exercise.verses);
    
    // Step 2: Extract the selected chapter name and ayah
    let selected_chapter_name = exercise.verses[selected_verse_index].verse.sourate.clone();
    let selected_ayah = exercise.verses[selected_verse_index].verse.clone();

    // Step 3: Extract and shuffle other chapter names and ayahs
    let mut other_chapter_data_set = std::collections::HashSet::new();
    let mut other_chapter_data = extract_and_shuffle_options(&exercise.verses, 
        |statement| {
            let chapter_ayah = Some(Alternative {
                content: statement.verse.text.clone(),
                ayah: Some(statement.verse.clone())
            });
            if other_chapter_data_set.insert(chapter_ayah.clone()) {
                chapter_ayah
            } else {
                None
            }
        }, 
        &Some(Alternative {
            content: selected_chapter_name.as_ref().unwrap_or(&"".to_string()).clone(),
            ayah: Some(selected_ayah.clone())
        }));
    
    // Limit to 3 possible answers (minus the correct answer which we will add back in Step 5)
    other_chapter_data.truncate(2);

    // Step 4: Hide the sourate, chapter_no, and ayah of the selected verse
    let selected_statement = &mut exercise.verses[selected_verse_index];
    if let Some(sourate) = &mut selected_statement.verse.sourate {
        sourate.clear();
    }
    selected_statement.verse.chapter_no = 0; // You can set this to any default value
    //selected_statement.ayah = 0; // Set it to a default value

    // Step 5: Add the selected chapter data back to the list
    other_chapter_data.push(Alternative {
        content: selected_chapter_name.unwrap_or_default(),
        ayah: Some(selected_ayah),
    });

    Some((selected_statement.clone(), other_chapter_data, ExerciseType::B))
}
