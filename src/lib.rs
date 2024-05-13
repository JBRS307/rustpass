use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;

use self::files::get_home_dir;

pub mod arguments;
pub mod ops;
pub mod encryption;
pub mod files;

const CONFIG_FILE: &'static str = ".pass_config";
const KEY_DIRECTORY: &'static str = ".pass_key";

pub fn init_config() -> Result<()> {
    let config_path = PathBuf::from(CONFIG_FILE);
    let exists = Path::try_exists(&config_path)?;

    if !exists {
        let mut pass_key_path = get_home_dir()?;
        pass_key_path.push(KEY_DIRECTORY);
        fs::write(&config_path, pass_key_path.to_str().expect("String conversion failure"))?;
        fs::create_dir_all(&pass_key_path)?;
    }
    Ok(())
}