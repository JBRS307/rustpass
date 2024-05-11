use std::fs;
use std::path::PathBuf;
// use anyhow::Result;

const STORAGE_FOLDER: &'static str = ".pass_storage";

fn get_storage_dir() -> PathBuf {
    let mut dir = dirs::home_dir().expect("Unable to get the home directory!");
    dir.push(STORAGE_FOLDER);
    dir
}

pub fn create_dir_tree(subfolder: &Option<PathBuf>, key_id: &String) {
    let mut path = get_storage_dir();

    if let Some(x) = subfolder {
        path.push(x);
    }

    path.push(key_id);

    fs::create_dir_all(path).expect("Directory creation error");
}

pub fn clear_dir(subfolder: &Option<PathBuf>) {
    let mut path = get_storage_dir();

    if let Some(x) = subfolder {
        path.push(x);
    }

    fs::remove_dir_all(path).expect("Directory removal error");
}