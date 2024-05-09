use anyhow::Result;
use self::common_func::*;

mod common_func;

pub fn add(
    page: &String,
    username: &String,
    password: &String,
    repeat_password: &String,
    copy: bool,
) -> Result<()> {
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



