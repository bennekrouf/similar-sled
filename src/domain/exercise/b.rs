use rand::prelude::SliceRandom;

use crate::models::{ExerciseType, Statement, Database, Alternative, Exercise};
use crate::domain::exercise::extract_and_shuffle_options::extract_and_shuffle_options;
use crate::domain::exercise::select_random_verse_index::select_random_verse_index;
use crate::domain::exercise::get_solution::get_solution;
use crate::domain::similar::sourate_from_verse::sourate_name_from_verse;
use crate::utils::deduplicate_by_field::deduplicate_by_field;
use crate::utils::extract_parts::extract_parts;

pub fn generate(dbs: &Database, kalima: String) -> Option<Exercise> {
    let mut exercises = get_solution(dbs, &kalima);
    if exercises.is_empty() { return None; }

    let exercise = &mut exercises[0];

    // Step 1: Get the index of the random verse
    let selected_verse_index = select_random_verse_index(&exercise.verses);
    
    // Step 2: Extract the selected chapter name and verse_no
    let selected_sourate = exercise.verses[selected_verse_index].verse.sourate.clone();
    let selected_ayah = exercise.verses[selected_verse_index].verse.clone();

    // Step 3: Extract and shuffle other chapter names and ayahs
    let mut other_chapter_data_set = std::collections::HashSet::new();
    let alternatives = extract_and_shuffle_options(&mut exercise.verses, 
        |statement: &mut Statement| {
            statement.verse.sourate = Some(sourate_name_from_verse(dbs, &statement.verse));
            statement.verse.ungrouped_text = Some(extract_parts(statement.verse.text.as_ref().map(String::as_str)));

            let chapter_ayah = Some(Alternative {
                content: statement.verse.sourate.as_ref().unwrap().to_string(),
                verse: Some(statement.verse.clone())
            });
            if other_chapter_data_set.insert(chapter_ayah.clone()) {
                chapter_ayah
            } else {
                None
            }
        }, 
        &Some(Alternative {
            content: selected_sourate.as_ref().unwrap_or(&"".to_string()).clone(),
            verse: Some(selected_ayah.clone())
        }));

    let mut distinct_alternatives = deduplicate_by_field(alternatives, |alt| alt.content.clone());
    distinct_alternatives.shuffle(&mut rand::thread_rng());
    
    // Limit to 3 possible answers (minus the correct answer which we will add back in Step 5)
    distinct_alternatives.truncate(2);

    // Step 4: Hide the sourate, chapter_no, and verse_no of the selected verse
    let selected_statement = &mut exercise.verses[selected_verse_index];
    if let Some(sourate) = &mut selected_statement.verse.sourate {
        sourate.clear();
    }
    selected_statement.verse.chapter_no = 0; // You can set this to any default value
    //selected_statement.chapter_no = 0; // Set it to a default value

    // Step 5: Add the selected chapter data back to the list
    distinct_alternatives.push(Alternative {
        content: selected_sourate.unwrap_or_default(),
        verse: Some(selected_ayah),
    });
     Some(Exercise {
        statement: selected_statement.clone(),
        alternatives: distinct_alternatives,
        exercise_type: ExerciseType::B
    })
}
