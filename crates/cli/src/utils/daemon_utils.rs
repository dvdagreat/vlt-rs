use vlt_crypto::Crypto;
use rpassword::prompt_password as ask_password;
use secrecy::SecretString;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub fn prompt_password(prompt: &str) -> SecretString {
    let pass = ask_password(prompt).unwrap();
    SecretString::new(pass.trim().to_owned().into())
}

pub fn get_master_key_from_user() -> [u8; 32] {
    let key = if let Some(cached_key) = get_key_from_daemon() {
        println!("Using master password from cache...");
        cached_key
    } else {
        // 2. Fallback to user prompt
        let password = prompt_password("Enter Master Password: ");
        let derived_key = Crypto::derive_key(&password);

        // 3. Save to daemon for future use (Start Session)
        save_key_to_daemon(&derived_key);
        derived_key
    };

    key
}

pub fn get_key_from_daemon() -> Option<[u8; 32]> {
    let mut stream = UnixStream::connect("/tmp/cred_manager.sock").ok()?;
    // Send exactly "GET"
    stream.write_all(b"GET").ok()?;

    let mut buf = [0u8; 32];
    // If the daemon sends back 32 bytes, we have a hit
    match stream.read_exact(&mut buf) {
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}

pub fn save_key_to_daemon(key: &[u8; 32]) {
    if let Ok(mut stream) = UnixStream::connect("/tmp/cred_manager.sock") {
        let mut payload = b"SET ".to_vec();
        payload.extend_from_slice(key);
        let _ = stream.write_all(&payload);
    }
}
