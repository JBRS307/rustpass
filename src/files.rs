use std::fs;
use std::path::{PathBuf, Path};
use anyhow::{Error, Result};

use crate::CONFIG_FILE;

const STORAGE_FOLDER: &'static str = ".pass_storage";

pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("Unable to get home directory")
}

pub fn get_storage_dir() -> PathBuf {
    let mut dir = get_home_dir();
    dir.push(STORAGE_FOLDER);
    dir
}

pub fn get_key_dir() -> PathBuf {
    PathBuf::from(fs::read_to_string(CONFIG_FILE).expect("File reading error"))
}

pub fn create_dir_tree(path: &PathBuf) {
    fs::create_dir_all(path).expect("Directory creation error");
}

pub fn clear_dir(path: &PathBuf) {
    fs::remove_dir_all(path).expect("Directory removal error");
}