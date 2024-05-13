use std::path::{PathBuf, Path};
use std::fs;
use std::process::exit;

use crate::files::get_storage_dir;

pub fn update_password(pass: &[u8], name: &String, subfolder: &Option<PathBuf>) {
    let mut pass_dir = get_storage_dir();

    if let Some(p) = subfolder {
        pass_dir.push(p);
    }

    if !Path::try_exists(&pass_dir).expect("Failed to check if file exists") {
        eprintln!("No such file or directory!");
        exit(1);
    }

    pass_dir.push(name);

    fs::write(&pass_dir, pass).expect("Failed to save password");
}