use std::fs;
use std::process::{self, Command};
use std::path::{Path, PathBuf};
use anyhow::{Result, Error};

use crate::files::*;
use crate::{STORAGE_FOLDER, KEY_DIRECTORY, TEMP_DIR};

const KEY_ARCHIVE: &str = "rustpass-keys.tar.gz";
const STORAGE_ARCHIVE: &str = "rustpass-storage.tar.gz";

const KEYS_OLD: &str = ".pass_keys_old";
const STORAGE_OLD: &str = ".pass_storage_old";

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
        return Err(Error::msg("Key error: Directory doesn't exist!"));
    }

    dir.pop();
    output.push(KEY_ARCHIVE);

   pack(&dir, &output, true)
}

pub fn pack_stoarge(mut output: PathBuf) -> Result<()> {
    let mut dir = get_storage_dir()?;

    if !Path::try_exists(&dir)? {
        return Err(Error::msg("Storage error: Directory doesn't exist!"));
    }

    dir.pop();
    output.push(STORAGE_ARCHIVE);

    pack(&dir, &output, false)
}

pub fn unpack(path: &PathBuf) -> Result<()> {
    // At this point path should be already validated

    let mut output_dir = get_home_dir()?;
    output_dir.push(TEMP_DIR);
    create_dir_tree(&output_dir)?;

    let status = Command::new("tar")
        .args([
            "-xf",
            path.to_str().expect("String conversion error"),
            "-C",
            output_dir.to_str().expect("String conversion error"),
        ])
        .stdout(process::Stdio::null())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::msg(format!("Tar extraction error: {}", status)))
    }
}

fn delete_temp() -> Result<()> {
    let mut temp = get_home_dir()?;
    temp.push(TEMP_DIR);
    if Path::try_exists(&temp)? {
        fs::remove_dir_all(temp)?;
    }
    Ok(())
}

fn delete_old_keys() -> Result<()> {
    let mut key_old = get_key_dir()?;
    key_old.pop();
    key_old.push(KEYS_OLD);

    if Path::try_exists(&key_old)? {
        fs::remove_dir_all(key_old)?;
    }

    Ok(())
}

fn delete_old_storage() -> Result<()> {
    let mut storage_old = get_storage_dir()?;
    storage_old.pop();
    storage_old.push(STORAGE_OLD);

    if Path::try_exists(&storage_old)? {
        fs::remove_dir_all(storage_old)?;
    }

    Ok(())
}

fn restore_old_keys() -> Result<()> {
    let key_dir = get_key_dir()?;
    let mut key_old = key_dir.parent().expect("Key dir error").to_path_buf();
    key_old.push(KEYS_OLD);

    fs::rename(&key_old, &key_dir)?;
    Ok(())
}

fn restore_old_storage() -> Result<()> {
    let storage_dir = get_key_dir()?;
    let mut storage_old = storage_dir.parent().expect("Storage dir error").to_path_buf();
    storage_old.push(STORAGE_OLD);

    fs::rename(&storage_old, &storage_dir)?;
    Ok(())
}

pub fn import_keys(new_keys: PathBuf) -> Result<()> {
    let key_dir = get_key_dir()?;
    let mut key_old = key_dir.parent().expect("Key dir error").to_path_buf();

    key_old.push(KEYS_OLD);

    if let Err(err) = fs::rename(&key_dir, &key_old) {
        delete_temp()?;
        return Err(Error::new(err));
    }

    match fs::rename(new_keys, key_dir) {
        Ok(_) => {
            delete_temp()?;
            delete_old_keys()?;
            Ok(())
        },
        Err(err) => {
            delete_temp()?;
            restore_old_keys()?;
            Err(Error::new(err))
        },
    }
}

pub fn import_storage(new_storage: PathBuf) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let mut storage_old = storage_dir.parent().expect("Storage dir error").to_path_buf();

    storage_old.push(STORAGE_OLD);

    if let Err(err) = fs::rename(&storage_dir, &storage_old) {
        delete_temp()?;
        return Err(Error::new(err));
    }

    match fs::rename(new_storage, storage_dir) {
        Ok(_) => {
            delete_temp()?;
            delete_old_storage()?;
            Ok(())
        },
        Err(err) => {
            delete_temp()?;
            restore_old_storage()?;
            Err(Error::new(err))
        },
    }
}