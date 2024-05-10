use rand::Rng;
use rand::distributions::{Alphanumeric, Uniform};

pub fn generate_alphanum(length: u32) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

pub fn generate_full(length: u32) -> String {
    // Symbols from 33 to 126 included are printable
    // ASCII symbols

    rand::thread_rng()
        .sample_iter(&Uniform::from(33..=126))
        .take(length as usize)
        .map(char::from)
        .collect()
}