use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
// use rand::{rngs::OsRng};
use rand_core::OsRng;
use secrecy::{ExposeSecret, SecretString};

pub struct CryptoHandler;

impl CryptoHandler {
    // Derive a 32-byte key for AES-256 using Argon2id
    pub fn derive_key(password: &SecretString, salt: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        let argon2 = Argon2::default();
        // Simplified for example; in production, use a specific salt
        let _ = argon2.hash_password_into(password.expose_secret().as_bytes(), salt, &mut key);
        key
    }

    pub fn encrypt(data: &str, key: &[u8; 32]) -> (Vec<u8>, [u8; 12]) {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, data.as_bytes())
            .expect("encryption failure!");
        (ciphertext, nonce_bytes)
    }

    pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> String {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Nonce::from_slice(nonce);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .expect("decryption failure!");
        String::from_utf8(plaintext).unwrap()
    }
}

pub struct Crypto;

impl Crypto {
    pub fn hash_password(password: &SecretString) -> String {
        let salt = SaltString::generate(OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.expose_secret().as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }

    pub fn verify_password(password: &SecretString, hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
        Argon2::default()
            .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn derive_key(password: &SecretString) -> [u8; 32] {
        let mut key = [0u8; 32];
        let salt = b"static_salt_for_demo"; // In production, store a unique salt in DB
        let _ = Argon2::default().hash_password_into(
            password.expose_secret().as_bytes(),
            salt,
            &mut key,
        );
        key
    }

    pub fn encrypt(data: &str, key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, data.as_bytes()).unwrap();
        (ciphertext, nonce_bytes.to_vec())
    }

    pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8]) -> String {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Nonce::from_slice(nonce);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .expect("Decryption failed - wrong key?");
        String::from_utf8(plaintext).unwrap()
    }
}
