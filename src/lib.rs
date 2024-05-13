use std::path::{Path, PathBuf};
use files::get_home_dir;
use std::fs;

pub mod arguments;
pub mod ops;
pub mod encryption;
pub mod files;

pub const CONFIG_FILE: &'static str = ".pass_config";
const KEY_DIRECTORY: &'static str = ".pass_key";

pub fn init_config() {
    let config_path = PathBuf::from(CONFIG_FILE);
    let exists = Path::try_exists(&config_path).expect("File existance check error");

    if !exists {
        let mut pass_key_path = get_home_dir();
        pass_key_path.push(KEY_DIRECTORY);
        fs::write(&config_path, pass_key_path.to_str().expect("String conversion error")).expect("Config file creation error");
        fs::create_dir_all(&pass_key_path).expect(".pass_key directory creation error");
    }
}