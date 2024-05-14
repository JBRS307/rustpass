use std::fs;
use std::path::PathBuf;
use anyhow::{Error, Result};
use dirs;

use crate::CONFIG_FILE;

pub const STORAGE_FOLDER: &'static str = ".pass_storage";

pub fn get_home_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir();

    match home_dir {
        Some(dir) => Ok(dir),
        None => Err(Error::msg("Failed to get current directory!")),
    }
}

pub fn get_storage_dir() -> Result<PathBuf> {
    let mut dir = get_home_dir()?;
    dir.push(STORAGE_FOLDER);
    Ok(dir)
}

pub fn get_key_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(fs::read_to_string(CONFIG_FILE)?))
}

pub fn create_dir_tree(path: &PathBuf) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn clear_dir(path: &PathBuf) -> Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}