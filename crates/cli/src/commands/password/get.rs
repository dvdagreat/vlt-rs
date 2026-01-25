use core::Crypto;

use arboard::Clipboard;
use std::thread;
use std::time::Duration;
use storage::Db;

use crate::utils::daemon_utils::get_master_key_from_user;

pub fn handler(db: &Db, service: String, identifier: String) {
    // 1. Try to get key from Daemon first
    let key = get_master_key_from_user();

    // 4. Proceed with decryption
    if let Ok(cred) = db.get_credential(&service, &identifier) {
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
    } else {
        println!("Error: No credential found for service `{}`.", service);
    }
}
