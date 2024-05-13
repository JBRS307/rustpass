use std::fs;
use std::path::{PathBuf, Path};
use anyhow::{Error, Result};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::files::get_key_dir;
use super::KEY_FILE;

pub fn copy_to_clipboard(text: &str) {
    let ctx: Result<ClipboardContext, Box<dyn std::error::Error>> = ClipboardProvider::new();

    match ctx {
        Ok(mut clp) => {
            clp.set_contents(text.to_owned()).unwrap_or_else(|err| {
                eprintln!("A clipboard error has occured: {}", err);
            });

            clp.get_contents().unwrap_or_else(|err| {
                eprintln!("A clipboard error has occured: {}", err);
                String::new()
            });
        },
        Err(err) => eprintln!("A clipboard error has occured: {}", err),
    }
}

pub fn get_key(subfolder: &Option<PathBuf>) -> Result<Vec<u8>> {
    let mut key_dir = get_key_dir()?;

    if let Some(p) = subfolder {
        key_dir.push(p);
    }

    if !Path::try_exists(&key_dir)? {
        return Err(Error::msg("Key doesn't exist!"));
    }

    key_dir.push(KEY_FILE);
    Ok(fs::read(key_dir)?)
}