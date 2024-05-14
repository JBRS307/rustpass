use std::process::Command;
use std::path::Path;
use anyhow::{Result, Error};

use crate::files::*;

pub fn pack(keys: bool) -> Result<()> {
    let dir = if keys {
        let mut buf = get_key_dir()?;
        buf.pop();
        buf
    } else {
        let mut buf = get_storage_dir()?;
        buf.pop();
        buf
    };

    if !Path::try_exists(&dir)? {
        return Err(Error::msg("No such file or directory!"));
    }

    Command::new("tar")
        .args([
            "-zcvf",
            "password.tar.gz",
            STORAGE_FOLDER,
        ])
        .output()?;

    Ok(())
}