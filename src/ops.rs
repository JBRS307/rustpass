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

pub fn init(subfolder: &Option<PathBuf>) -> Result<()> {
    let key = generate_key();
    let mut key_directory = get_key_dir()?;
    let mut storage_directory = get_storage_dir()?;
    

    if let Some(p) = subfolder {
        key_directory.push(p);
        storage_directory.push(p);
    }

    key_directory.push(KEY_FILE); // for the sake of existance check
    if Path::try_exists(&key_directory)? {
        return Err(Error::msg("Directory already initialized!"));
    }
    key_directory.pop();


    create_dir_tree(&storage_directory)?;
    create_dir_tree(&key_directory)?;

    key_directory.push(KEY_FILE);
    fs::write(&key_directory, key)?;
    Ok(())
}

pub fn add(
    subfolder: &Option<PathBuf>,
    name: &String,
    password: &String,
    repeat_password: &String,
    copy: bool,
) -> Result<()> {
    if password.ne(repeat_password) {
        return Err(Error::msg("Passwords differ"));
    }

    let key = get_key(subfolder)?;
    let encrypted = encrypt(&key, password);

    add_new_password(&encrypted, name, subfolder)?;

    if copy {
        copy_to_clipboard(password);
    }
    println!("Password added successfully!");
    Ok(())
}

pub fn update(
    subfolder: &Option<PathBuf>,
    name: &String,
    password: &String,
    repeat_password: &String,
    copy: bool,
) -> Result<()> {
    if password.ne(repeat_password) {
        return Err(Error::msg("Passwords differ"));
    }

    let key = get_key(subfolder)?;
    let encrypted = encrypt(&key, password);

    update_password(&encrypted, name, subfolder)?;

    if copy {
        copy_to_clipboard(password);
    }

    println!("Password updated successfully!");
    Ok(())
}

pub fn remove(
    subfolder: &Option<PathBuf>,
    name: &String,
    copy: bool,
) -> Result<()> {
    let mut path = get_storage_dir()?;
    if let Some(p) = subfolder {
        path.push(p);
    }
    path.push(name);

    if !Path::try_exists(&path)? {
        return Err(Error::msg("No such file or directory!"));
    }

    if copy {
        get(subfolder, name, true, true)?;
    }

    fs::remove_file(&path)?;

    println!("Password removed successfully!");
    Ok(())
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
) -> Result<()> {
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
        let key = get_key(subfolder)?;
        let encrypted = encrypt(&key, &password);
        
        if new_save {
            add_new_password(&encrypted, &name.clone().unwrap(), subfolder)?;
        } else if save {
            update_password(&encrypted, &name.clone().unwrap(), subfolder)?;
        }
    }

    Ok(())
}

pub fn get(
    subfolder: &Option<PathBuf>,
    name: &String,
    no_print: bool,
    copy: bool,
) -> Result<()> {
    let mut path = get_storage_dir()?;

    if let Some(p) = subfolder {
        path.push(p);
    }
    path.push(name);

    if !Path::try_exists(&path)? {
        return Err(Error::msg("No such file or directory!"))
    }

    let encrypted = fs::read(&path)?;
    let key = get_key(subfolder)?;

    let password = decrypt(&key, &encrypted);

    if copy {
        copy_to_clipboard(&password);
    }

    if !no_print {
        println!("{}", password);
    }
    Ok(())
}

pub fn list(subfolder: &Option<PathBuf>) -> Result<()> {
    let mut arg = get_storage_dir()?;

    if let Some(p) = subfolder {
        arg.push(p);
    }

    if !Path::try_exists(&arg)? {
        return Err(Error::msg("No such file or directory!"));
    }

    Command::new("tree")
        .current_dir(&arg)
        .spawn()?;
    Ok(())
}

pub fn clear(subfolder: &Option<PathBuf>) -> Result<()> {
    let mut storage_dir = get_storage_dir()?;
    let mut key_dir = get_key_dir()?;

    if let Some(p) = subfolder {
        storage_dir.push(p);
        key_dir.push(p);
    }

    if !Path::try_exists(&storage_dir)? {
        return Err(Error::msg("No such file or directory!"));
    }

    clear_dir(&storage_dir)?;
    clear_dir(&key_dir)?;

    Ok(())
}

pub fn config(path: &Option<PathBuf>, get: bool, reset: bool) -> Result<()> {
    if get {
        println!("{}", fs::read_to_string(CONFIG_FILE)?);
    }

    if reset {
        let old_key_dir = get_key_dir()?;
        let mut new_key_dir = get_home_dir()?;
        new_key_dir.push(KEY_DIRECTORY);

        change_config(&old_key_dir, &new_key_dir)?;
        Ok(())
    } else if let Some(p) = path {
        let old_key_dir = get_key_dir()?;
        let mut new_key_dir = p.to_owned();

        if !Path::try_exists(&new_key_dir)? {
            create_dir_tree(&new_key_dir)?;
        }

        new_key_dir.push(KEY_DIRECTORY);

        change_config(&old_key_dir, &new_key_dir)?;
        Ok(())
    } else {
        Ok(())
    }
}

pub fn git(args: &Option<Vec<String>>, clear: bool) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    if !Path::try_exists(&storage_dir)? {
        return Err(Error::msg("Storage uninitialized!"))
    }

    if clear {
        remove_repo(storage_dir)?;
        println!("Successfully removed repository!");
        Ok(())
    } else {
        let arg_vec = match args {
            Some(vec) => vec.to_owned(),
            None => Vec::new(),
        };
    
        let status = Command::new("git")
            .current_dir(&storage_dir)
            .args(&arg_vec)
            .status()?;
    
            if status.success() {
                println!("Execution successful!");
                Ok(())
            } else {
                Err(Error::msg(format!("\"git\" error: {}", status)))
            }
    }
}



