use openssl::symm;
use rand::{distributions::Standard, Rng};

const KEY_LENGTH: usize = 32;

pub fn encrypt(key: &[u8], pass: &String) -> Vec<u8> {
    let cipher = symm::Cipher::aes_256_cbc();
    symm::encrypt(cipher, key, None, pass.as_bytes()).expect("Encryption error")
}

pub fn decrypt(key: &[u8], pass: &Vec<u8>) -> String {
    let cipher = symm::Cipher::aes_256_cbc();
    let decrypted = symm::decrypt(cipher, key, None, pass).expect("Decryption error");
    std::str::from_utf8(&decrypted).expect("Incorect string after decryption").to_string()
}

pub fn generate_key() -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(&Standard)
        .take(KEY_LENGTH)
        .collect()
}

