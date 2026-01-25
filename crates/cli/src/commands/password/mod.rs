pub mod add;

pub mod edit;

pub mod get;

pub mod rm;

// pub fn get_command(db: &Db, service: String, identifier: String) {
//     // 1. Try to get key from Daemon first
//     let key = get_master_key_from_user();

//     // 4. Proceed with decryption
//     if let Ok(cred) = db.get_credential(&service, &identifier) {
//         let decrypted = Crypto::decrypt(&cred.secret, &key, &cred.nonce);

//         let mut cb = Clipboard::new().expect("Clipboard: Cannot connect to clipboard");
//         cb.set_text(decrypted).unwrap();
//         println!(
//             "Clipboard: Password for `{}` on `{}` copied to clipboard.",
//             cred.identifier, service
//         );

//         // Clipboard auto-clear thread
//         thread::spawn(move || {
//             thread::sleep(Duration::from_secs(15));
//             if let Ok(mut cb_internal) = Clipboard::new() {
//                 let _ = cb_internal.set_text("".to_string());
//             }
//         });
//     } else {
//         println!("Error: No credential found for service `{}`.", service);
//     }
// }

// pub fn add_command(db: &Db, service: String, username: String, secret: String) {
//     let key = get_master_key_from_user();
//     if db.get_credential(&service, &username).is_ok() {
//         println!(
//             "Error: Credential for `{}` on `{}` already exists. Use the edit command to modify it.",
//             username, service
//         );
//         return;
//     }

//     let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
//     db.add_credential(&service, &username, &encrypted, &nonce)
//         .unwrap();
//     println!(
//         "Success: Successfully saved `{}` credential for `{}`.",
//         service, username
//     );
// }

// pub fn edit_command(db: &Db, service: String, username: String, secret: String) {
//     let key = get_master_key_from_user();
//     if db.get_credential(&service, &username).is_err() {
//         println!(
//             "Error: No existing credential for `{}` on `{}`. Use the add command to create it first.",
//             username, service
//         );
//         return;
//     }

//     let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
//     db.edit_credential(&service, &username, &encrypted, &nonce)
//         .unwrap();
//     println!(
//         "Success: Successfully edited `{}` credential for `{}`.",
//         service, username
//     );
// }

// pub fn rm_command(db: &Db, service: String, identifier: String) {
//     let key = get_master_key_from_user();

//     if key.is_empty() {
//         println!("Error: Master key is required to remove credentials.");
//         return;
//     }

//     if db.get_credential(&service, &identifier).is_err() {
//         println!(
//             "Error: No existing credential for `{}` on `{}`.",
//             identifier, service
//         );
//         return;
//     }

//     db.delete_credential(&service, &identifier).unwrap();
//     println!(
//         "Success: Successfully removed `{}` credential for `{}`.",
//         service, identifier
//     );
// }
