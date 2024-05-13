use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::str;
use anyhow::{Result, Error};

use self::common_func::*;
use self::generate_func::*;
use self::config_func::*;
use self::add_func::*;
use self::update_func::*;
use self::git_func::*;

use crate::encryption::*;
use crate::files::*;
use crate::{CONFIG_FILE, KEY_DIRECTORY};

mod common_func;
mod generate_func;
mod config_func;
mod add_func;
mod update_func;
mod git_func;

const KEY_FILE: &str = "key";

pub fn init(subfolder: &Option<PathBuf>) {
    let key = generate_key();
    let mut key_directory = get_key_dir();
    let mut storage_directory = get_storage_dir();
    

    if let Some(p) = subfolder {
        key_directory.push(p);
        storage_directory.push(p);
    }

    key_directory.push(KEY_FILE); // for the sake of existance check
    if Path::try_exists(&key_directory).expect("Directory existance check error") {
        eprintln!("Directory already initialized!");
        exit(1);
    }
    key_directory.pop();


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
        eprintln!("Passwords differ!");
        exit(1);
    }

    let key = get_key(subfolder);
    let encrypted = encrypt(&key, password);

    add_new_password(&encrypted, name, subfolder);

    if copy {
        copy_to_clipboard(password);
    }
    println!("Password added successfully!");
}

pub fn update(
    subfolder: &Option<PathBuf>,
    name: &String,
    password: &String,
    repeat_password: &String,
    copy: bool,
)
{
    if password.ne(repeat_password) {
        eprintln!("Passwords differ!");
        exit(1);
    }

    let key = get_key(subfolder);
    let encrypted = encrypt(&key, password);

    update_password(&encrypted, name, subfolder);

    if copy {
        copy_to_clipboard(password);
    }

    println!("Password updated successfully!");
}

pub fn remove(
    subfolder: &Option<PathBuf>,
    name: &String,
    copy: bool,
)
{
    let mut path = get_storage_dir();
    if let Some(p) = subfolder {
        path.push(p);
    }
    path.push(name);

    if !Path::try_exists(&path).expect("Failed to check if file exists") {
        eprintln!("No such file or directory!");
        exit(1);
    }

    if copy {
        get(subfolder, name, true, true);
    }

    fs::remove_file(&path).expect("Error removing file");

    println!("Password removed successfully!");
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

    if save || new_save {
        let key = get_key(subfolder);
        let encrypted = encrypt(&key, &password);
        
        if new_save {
            add_new_password(&encrypted, &name.clone().unwrap(), subfolder);
        } else if save {
            update_password(&encrypted, &name.clone().unwrap(), subfolder);
        }
    }

}

pub fn get(
    subfolder: &Option<PathBuf>,
    name: &String,
    no_print: bool,
    copy: bool,
)
{
    let mut path = get_storage_dir();

    if let Some(p) = subfolder {
        path.push(p);
    }
    path.push(name);

    if !Path::try_exists(&path).expect("Failed to check if file exists") {
        eprintln!("No such file or directory!");
        exit(1);
    }

    let encrypted = fs::read(&path).expect("Error reading from file");
    let key = get_key(subfolder);

    let password = decrypt(&key, &encrypted);

    if copy {
        copy_to_clipboard(&password);
    }

    if !no_print {
        println!("{}", password);
    }
}

pub fn list(subfolder: &Option<PathBuf>) {
    let mut arg = get_storage_dir();

    if let Some(p) = subfolder {
        arg.push(p);
    }

    if !Path::try_exists(&arg).expect("Failed to check if file exists") {
        eprintln!("No such file or directory!");
        exit(1);
    }

    Command::new("tree")
        .current_dir(&arg)
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("Couldn't run \"tree\": {}", err);
            exit(1);
        });
}

pub fn clear(subfolder: &Option<PathBuf>) {
    let mut storage_dir = get_storage_dir();
    let mut key_dir = get_key_dir();

    if let Some(p) = subfolder {
        storage_dir.push(p);
        key_dir.push(p);
    }

    if !Path::try_exists(&storage_dir).expect("Failed to check if file exists") {
        Err(Error::msg("No such file or directory!"))
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

pub fn git(args: &Option<String>) {
    let args_vec = match args {
        Some(s) => parse_args(s),
        None => Vec::new(),
    };

    let storage_dir = get_storage_dir();
    if !Path::try_exists(&storage_dir).expect("Failed to check if directory exists") {
        eprintln!("Storage uninitialized!");
        exit(1);
    }

    let status = Command::new("git")
        .current_dir(&storage_dir)
        .args(&args_vec)
        .status()
        .unwrap_or_else(|err| {
            eprintln!("Coudln't execute \"git\" command: {}", err);
            exit(1);
        });

        if status.success() {
            println!("Execution successful!");
        } else {
            eprintln!("Failed to execute \"git\": {}", status);
        }
}



