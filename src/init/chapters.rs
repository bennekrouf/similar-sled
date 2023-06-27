use crate::files::chapters_from_yaml::load;
use crate::db::chapter;
use crate::utils::yml_path;

pub fn init(db: &sled::Db) {
    if db.is_empty() {
        let data_folder_path = yml_path::get_data_folder_path();
        let chapters = load(data_folder_path.join("chapters.yaml").to_str().unwrap());
        for chapter in chapters {
            chapter::insert(&db, &chapter).unwrap();
        }
    }
}