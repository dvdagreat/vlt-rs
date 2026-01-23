use arboard::Clipboard;
use clap::{Parser, Subcommand};
use core::Crypto;
use daemon_utils::get_master_key_from_user;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use storage::Db;

mod daemon_utils;

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
    let db = Db::init("vault.db").expect("Failed to access the credential store");

    match cli.command {
        Commands::Add {
            service,
            username,
            secret,
        } => {
            let key = get_master_key_from_user();
            let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
            db.add_credential(&service, &username, &encrypted, &nonce)
                .unwrap();
            println!(
                "Success: Successfully saved `{}` credential for `{}`.",
                service, username
            );
        }
        Commands::Get { service } => {
            // 1. Try to get key from Daemon first
            let key = get_master_key_from_user();

            // 4. Proceed with decryption
            if let Ok(cred) = db.get_credential(&service) {
                let decrypted = Crypto::decrypt(&cred.secret, &key, &cred.nonce);

                let mut cb = Clipboard::new().expect("Clipboard: Cannot connect to clipboard");
                cb.set_text(decrypted).unwrap();
                println!(
                    "Clipboard: Password for `{}` on `{}` copied to clipboard.",
                    cred.identifier, service
                );

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
            print!("DANGER ZONE: Are you ABSOLUTELY sure you want to erase EVERYTHING? (y/N): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                db.delete_all_data().unwrap();
                println!("Success: All passwords dropped from the database.");
            }
        }
    }
}
