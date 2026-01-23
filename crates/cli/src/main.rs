use arboard::Clipboard;
use clap::{Parser, Subcommand};
use core::Crypto;
use secrecy::SecretString;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::thread;
use std::time::Duration;
use storage::Db;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save a new credential
    Add {
        service: String,
        username: String,
        secret: String,
    },
    /// Copy credential to clipboard
    Get { service: String },
    /// Wipe all data and forget master password
    Nuke,
}

fn main() {
    let cli = Cli::parse();
    let db = Db::init("vault.db").expect("Failed to open DB");

    match cli.command {
        Commands::Add {
            service,
            username,
            secret,
        } => {
            let master = prompt_password("Enter Master Password: ");
            let key = Crypto::derive_key(&master);
            let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
            db.add_credential(&service, &username, &encrypted, &nonce)
                .unwrap();
            println!("✅ Saved {} successfully.", service);
        }
        Commands::Get { service } => {
            // 1. Try to get key from Daemon first
            let key = if let Some(cached_key) = get_key_from_daemon() {
                println!("🔓 Using cached session...");
                cached_key
            } else {
                // 2. Fallback to user prompt
                let password = prompt_password("Master Password: ");
                let derived_key = Crypto::derive_key(&password);

                // 3. Save to daemon for future use (Start Session)
                save_key_to_daemon(&derived_key);
                derived_key
            };

            // 4. Proceed with decryption
            if let Ok(cred) = db.get_credential(&service) {
                let decrypted = Crypto::decrypt(&cred.secret, &key, &cred.nonce);

                let mut cb = Clipboard::new().expect("Clipboard error");
                cb.set_text(decrypted).unwrap();
                println!("📋 Password for {} copied to clipboard.", service);

                // Clipboard auto-clear thread
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(15));
                    if let Ok(mut cb_internal) = Clipboard::new() {
                        let _ = cb_internal.set_text("".to_string());
                    }
                });
            }
        }
        Commands::Nuke => {
            print!("Are you sure you want to erase EVERYTHING? (y/N): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                db.delete_all_data().unwrap();
                println!("☢️ Vault shredded.");
            }
        }
    }
}

fn prompt_password(prompt: &str) -> SecretString {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut pass = String::new();
    io::stdin().read_line(&mut pass).unwrap();
    SecretString::new(pass.trim().to_owned().into())
}

fn get_key_from_daemon() -> Option<[u8; 32]> {
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

fn save_key_to_daemon(key: &[u8; 32]) {
    if let Ok(mut stream) = UnixStream::connect("/tmp/cred_manager.sock") {
        let mut payload = b"SET ".to_vec();
        payload.extend_from_slice(key);
        let _ = stream.write_all(&payload);
    }
}
