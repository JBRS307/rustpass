use anyhow::{Result, Error};
use self::common_func::*;
use self::generate_func::*;
use crate::encryption::*;

mod common_func;
mod generate_func;

pub fn init(path: &Option<String>, key_id: &String) -> Result<()> {
    let key = generate_key();
    Ok(())
}

pub fn add(
    page: &String,
    username: &String,
    password: &String,
    repeat_password: &String,
    copy: bool,
) -> Result<()> {
    if password.ne(repeat_password) {
        return Err(Error::msg("Passwords differ!"));
    }

    if copy {
        copy_to_clipboard(password);
    }

    Ok(())
}

pub fn update(
    page: &String,
    username: &String,
    password: &String,
    repeat_password: &String,
    copy_old: bool,
    copy_new: bool,
) -> Result<()> {
    if password.ne(repeat_password) {
        return Err(Error::msg("Passwords differ!"));
    }
    Ok(())
}

pub fn remove(
    page: &String,
    username: &String,
    copy: bool,
) -> Result<()> {
    Ok(())
}

pub fn generate(
    no_symbols: bool,
    copy: bool,
    no_print: bool,
    save: bool,
    new_save: bool,
    length: u32,
    page: &Option<String>,
    username: &Option<String>,
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

    Ok(())
}

pub fn get(
    page: &String,
    username: &String,
    no_print: bool,
    copy: bool,
) -> Result<()> {
    Ok(())
}

pub fn list(path: &Option<String>, recursive: bool) -> Result<()> {
    Ok(())
}



