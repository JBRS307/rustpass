use anyhow::Result;
use std::path::PathBuf;
use dirs;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

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

pub fn get_key() -> Vec<u8> {
    // TODO
    Vec::new()
}