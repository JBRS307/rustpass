use std::path::PathBuf;
use std::fs;
use anyhow::Result;

use crate::CONFIG_FILE;

pub fn change_config(old_dir: &PathBuf, new_dir: &PathBuf) -> Result<()> {
    fs::rename(old_dir, new_dir)?;
    fs::write(CONFIG_FILE, new_dir.to_str().expect("String conversion error"))?;
    Ok(())
}