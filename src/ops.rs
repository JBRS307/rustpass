use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::str;

use self::common_func::*;
use self::generate_func::*;
use self::config_func::*;
use crate::{encryption::*, KEY_DIRECTORY};
use crate::files::*;
use crate::CONFIG_FILE;

mod common_func;
mod generate_func;
mod config_func;

const KEY_FILE: &str = "key";

pub fn init(subfolder: &Option<PathBuf>) {
    let key = generate_key();
    let mut key_directory = get_key_dir();
    let mut storage_directory = get_storage_dir();
    

    if let Some(p) = subfolder {
        key_directory.push(p);
        storage_directory.push(p);
    }

    if Path::try_exists(&storage_directory).expect("Directory existance check error") {
        panic!("Directory already initialized!");
    }

    create_dir_tree(&storage_directory);
    create_dir_tree(&key_directory);

    key_directory.push(KEY_FILE);
    fs::write(&key_directory, key).expect("File writing error");
}

pub fn add(
    subfolder: &Option<PathBuf>,
    name: &String,
    password: &String,
    repeat_password: &String,
    copy: bool,
)
{
    if password.ne(repeat_password) {
        panic!("Passwords differ!");
    }

    let key = get_key();

    let encrypted = encrypt(&key, password);

    if copy {
        copy_to_clipboard(password);
    }
}

pub fn update(
    subfolder: &Option<PathBuf>,
    name: &String,
    password: &String,
    repeat_password: &String,
    copy_old: bool,
    copy_new: bool,
)
{
    if password.ne(repeat_password) {
        panic!("Password differ!");
    }
}

pub fn remove(
    subfolder: &Option<PathBuf>,
    name: &String,
    copy: bool,
)
{

}

pub fn generate(
    no_symbols: bool,
    copy: bool,
    no_print: bool,
    save: bool,
    new_save: bool,
    length: u32,
    subfolder: &Option<PathBuf>,
    name: &Option<String>,
)
{
    let password: String = if no_symbols {
        generate_alphanum(length)
    } else {
        generate_full(length)
    };

    if !no_print {
        println!("{}", password);
    }

    if copy {
        copy_to_clipboard(&password);
    }
}

pub fn get(
    subfolder: &Option<PathBuf>,
    name: &String,
    no_print: bool,
    copy: bool,
)
{

}

pub fn list(subfolder: &Option<PathBuf>) {
    let mut arg = get_storage_dir();

    if let Some(p) = subfolder {
        arg.push(p);
    }

    let output =  match Command::new("ls")
        .arg(arg)
        .output()
        {
            Ok(o) => o,
            Err(err) => panic!("{}", err),
        };
    
    println!("{}", str::from_utf8(&output.stdout).expect("String conversion error"));
}

pub fn clear(subfolder: &Option<PathBuf>) {
    let mut storage_dir = get_storage_dir();
    let mut key_dir = get_key_dir();

    if let Some(p) = subfolder {
        storage_dir.push(p);
        key_dir.push(p);
    }

    clear_dir(&storage_dir);
    clear_dir(&key_dir);
}

pub fn config(path: &Option<PathBuf>, get: bool, reset: bool) {
    if get {
        println!("{}", fs::read_to_string(CONFIG_FILE).expect("File reading error"));
    }

    if reset {
        let old_key_dir = get_key_dir();
        let mut new_key_dir = get_home_dir();
        new_key_dir.push(KEY_DIRECTORY);

        change_config(&old_key_dir, &new_key_dir);
    } else if let Some(p) = path {
        let old_key_dir = get_key_dir();
        let mut new_key_dir = p.to_owned();

        if !Path::try_exists(&new_key_dir).expect("File existance check error") {
            create_dir_tree(&new_key_dir);
        }

        new_key_dir.push(KEY_DIRECTORY);

        change_config(&old_key_dir, &new_key_dir);
    }
}



