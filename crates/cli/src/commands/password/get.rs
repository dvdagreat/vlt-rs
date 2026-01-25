use core::Crypto;

use arboard::Clipboard;
use std::thread;
use std::time::Duration;
use storage::Db;

use crate::utils::daemon_utils::get_master_key_from_user;
use crate::utils::selectable_utils::{get_selected_identifier, get_selected_service};

pub fn handler(db: &Db, flag_service: Option<String>, flag_identifier: Option<String>) {
    // 1. Try to get key from Daemon first
    let key = get_master_key_from_user();

    let selected_service = match flag_service {
        None => get_selected_service(db, None),
        Some(value_srv) => value_srv,
    };

    let selected_identifier = match flag_identifier {
        None => get_selected_identifier(db, selected_service.to_string(), None),
        Some(value_ident) => value_ident,
    };

    // 4. Proceed with decryption
    if let Ok(cred) = db.get_credential(&selected_service, &selected_identifier) {
        let decrypted = Crypto::decrypt(&cred.secret, &key, &cred.nonce);

        let mut cb = Clipboard::new().expect("Clipboard: Cannot connect to clipboard");
        cb.set_text(decrypted).unwrap();
        println!(
            "Clipboard: Password for `{}` on `{}` copied to clipboard.",
            cred.identifier, selected_service
        );

        // Clipboard auto-clear thread
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(15));
            if let Ok(mut cb_internal) = Clipboard::new() {
                let _ = cb_internal.set_text("".to_string());
            }
        });
    } else {
        println!(
            "Error: No credential found for service `{}`.",
            selected_service
        );
    }
}
