use core::Crypto;

use arboard::Clipboard;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use std::thread;
use std::time::Duration;
use storage::Db;

use crate::utils::daemon_utils::get_master_key_from_user;

pub fn handler(db: &Db) {
    // 1. Try to get key from Daemon first
    let key = get_master_key_from_user();

    let service_options = db
        .get_service_list()
        .expect("Cannot Fetch Selectable options");

    let service_selection_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("which service will you select?")
        .items(&service_options)
        .default(0)
        .interact()
        .expect("Cannot prepare a select list for you at the moment");

    // println!("You selected: {}", options[selection]);
    let selected_service = service_options[service_selection_idx].to_owned();

    let identifier_options = db
        .get_identifier_list_by_service_name(selected_service.to_string())
        .expect("Cannot Fetch Selectable options");

    let identifier_selection_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("which identifier will you select?")
        .items(&identifier_options)
        .default(0)
        .interact()
        .expect("Cannot prepare a select list for you at the moment");

    // println!("You selected: {}", options[selection]);
    let selected_identifier = identifier_options[identifier_selection_idx].to_owned();

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
