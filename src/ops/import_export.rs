use std::process::{self, Command};
use std::path::{Path, PathBuf};
use anyhow::{Result, Error};

use crate::files::*;
use crate::{STORAGE_FOLDER, KEY_DIRECTORY};

const KEY_ARCHIVE: &str = "rustpass-keys.tar.gz";
const STORAGE_ARCHIVE: &str = "rustpass-storage.tar.gz";

fn pack(dir: &PathBuf, output: &PathBuf, keys: bool) -> Result<()> {
    let folder = if keys {
        KEY_DIRECTORY
    } else {
        STORAGE_FOLDER
    };

    let status = Command::new("tar")
        .current_dir(&dir)
        .args([
            "-czf",
            output.to_str().expect("String conversion error"),
            folder,
        ])
        .stdout(process::Stdio::null())
        .status()?;

    if status.success() {
        println!("Archive created successfully!");
        Ok(())
    } else {
        Err(Error::msg(format!("Archive creation error: {}", status)))
    }
}

pub fn pack_keys(mut output: PathBuf) -> Result<()> {
    let mut dir = get_key_dir()?;

    if !Path::try_exists(&dir)? {
        return Err(Error::msg("Key directory doesn't exist!"));
    }

    dir.pop();
    output.push(KEY_ARCHIVE);

   pack(&dir, &output, true)
}

pub fn pack_stoarge(mut output: PathBuf) -> Result<()> {
    let mut dir = get_storage_dir()?;

    if !Path::try_exists(&dir)? {
        return Err(Error::msg("Storage directory doesn't exist!"));
    }

    dir.pop();
    output.push(STORAGE_ARCHIVE);

    pack(&dir, &output, false)
}