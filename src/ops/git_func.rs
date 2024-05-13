use std::path::{Path, PathBuf};
use anyhow::{Error, Result};
use std::fs;

const GIT_DIR: &str = ".git";
const GITIGNORE: &str = ".gitignore";

pub fn remove_repo(mut path: PathBuf) -> Result<()> {
    path.push(GIT_DIR);

    if !Path::try_exists(&path)? {
        return Err(Error::msg("Not a git repository!"));
    } else {
        fs::remove_dir_all(&path)?;
    }

    path.pop();
    path.push(GITIGNORE);
    if Path::try_exists(&path)? {
        fs::remove_file(&path)?;
    }
    Ok(())
}