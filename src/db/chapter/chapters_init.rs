use crate::files::chapters_from_yaml::load;
use crate::db::chapter::chapter_insert;
use crate::models::Database;
use crate::utils::yml_path;

pub fn init(dbs: &Database) {
    // if dbs.is_empty() {
        let data_folder_path = yml_path::get_data_folder_path();
        let chapters = load(data_folder_path.join("chapters.yaml").to_str().unwrap());
        for chapter in chapters {
            chapter_insert::insert(&dbs, &chapter).unwrap();
        }
    // }
}