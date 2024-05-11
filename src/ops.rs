use std::path::PathBuf;

use anyhow::{Result, Error};
use self::common_func::*;
use self::generate_func::*;
use crate::encryption::*;
use crate::files::*;

mod common_func;
mod generate_func;

pub fn init(path: &Option<PathBuf>, key_id: &String) {
    let key = generate_key();
    create_dir_tree(path, key_id);
}

pub fn add(
    path: &PathBuf,
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
    path: &PathBuf,
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
    path: &PathBuf,
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
    path: &Option<PathBuf>
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
    path: &PathBuf,
    no_print: bool,
    copy: bool,
)
{

}

pub fn list(path: &Option<PathBuf>) {

}

pub fn clear(path: &Option<PathBuf>) {
    clear_dir(path);
}



