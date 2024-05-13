use std::path::PathBuf;
use std::fs;

use crate::CONFIG_FILE;

pub fn change_config(old_dir: &PathBuf, new_dir: &PathBuf) {
    fs::rename(old_dir, new_dir).expect("Error while moving directory");
    fs::write(CONFIG_FILE, new_dir.to_str().expect("String conversion error")).expect("Writing to file error");
}