use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use pbkdf2::pbkdf2_hmac;
use ring::rand::{SecureRandom, SystemRandom};
use sha2::Sha256;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

pub fn generate_salt() -> Vec<u8> {
    let rng = SystemRandom::new();
    let mut salt = vec![0u8; SALT_LEN];
    rng.fill(&mut salt).expect("failed to generate salt");
    salt
}

pub fn hash_password(password: &str, salt: &[u8]) -> String {
    let key = derive_key(password, salt);
    hex::encode(key)
}

pub fn verify_password(password: &str, salt: &[u8], hash: &str) -> bool {
    let key = derive_key(password, salt);
    hex::encode(key) == hash
}

pub fn encrypt(plaintext: &str, master_key: &[u8; KEY_LEN], entry_id: &str) -> Result<String, String> {
    let sub_salt = format!("entry-{}-subkey", entry_id);
    let mut sub_key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(master_key, sub_salt.as_bytes(), 1, &mut sub_key);

    let rng = SystemRandom::new();
    let mut nonce_bytes = vec![0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes).expect("failed to generate nonce");
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&sub_key).map_err(|e| e.to_string())?;
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).map_err(|e| e.to_string())?;

    let mut result = nonce_bytes;
    result.extend_from_slice(&ciphertext);
    Ok(BASE64.encode(&result))
}

pub fn decrypt(encrypted: &str, master_key: &[u8; KEY_LEN], entry_id: &str) -> Result<String, String> {
    let sub_salt = format!("entry-{}-subkey", entry_id);
    let mut sub_key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(master_key, sub_salt.as_bytes(), 1, &mut sub_key);

    let data = BASE64.decode(encrypted).map_err(|e| e.to_string())?;
    if data.len() < NONCE_LEN {
        return Err("invalid ciphertext".to_string());
    }
    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&sub_key).map_err(|e| e.to_string())?;
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

pub fn generate_random_password(length: usize, use_uppercase: bool, use_lowercase: bool, use_digits: bool, use_symbols: bool) -> String {
    let mut charset = String::new();
    if use_uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if use_lowercase { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if use_digits { charset.push_str("0123456789"); }
    if use_symbols { charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?"); }
    if charset.is_empty() {
        charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    }
    let chars: Vec<char> = charset.chars().collect();
    let rng = SystemRandom::new();
    let mut password = String::new();
    let mut index_buf = [0u8; 1];
    for _ in 0..length {
        rng.fill(&mut index_buf).expect("rng failed");
        let idx = (index_buf[0] as usize) % chars.len();
        password.push(chars[idx]);
    }
    password
}
