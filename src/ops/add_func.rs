use std::path::{PathBuf, Path};
use std::fs;
use anyhow::{Result, Error};

use crate::files::get_storage_dir;

pub fn add_new_password(pass: &[u8], name: &String, subfolder: &Option<PathBuf>) -> Result<()> {
    let mut pass_dir = get_storage_dir()?;

    if let Some(p) = subfolder {
        pass_dir.push(p);
    }

    if !Path::try_exists(&pass_dir)? {
        return Err(Error::msg("No such file or directory!"));
    }

    pass_dir.push(name);

    if Path::try_exists(&pass_dir)? {
        return Err(Error::msg("Password already exists in this directory!"));
    }

    fs::write(&pass_dir, pass)?;
    Ok(())
}