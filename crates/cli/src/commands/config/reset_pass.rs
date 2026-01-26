use vlt_crypto::Crypto;

use vlt_store::Db;

use crate::utils::daemon_utils::{get_master_key_from_user, prompt_password, save_key_to_daemon};

pub fn handler(db: &Db) {
    // get master password
    let master_password = get_master_key_from_user();

    // get a record from stored credentials
    let credential_row = db.get_random_credential().expect("Internal Server Error");

    // try decrypting data using master password
    let decrypted = Crypto::decrypt(
        &credential_row.secret,
        &master_password,
        &credential_row.nonce,
    );

    // if it fails then show user error "Incorrect Master password!"
    if decrypted == "" {
        println!("Error: Incorrect Master Password!");
        return;
    }

    // prompt user to enter new master password
    let new_master_password = prompt_password("Enter New Master Password: ");
    let derived_new_key = Crypto::derive_key(&new_master_password);

    // get credential rows in batches of 20
    let batch_size = 20;
    let mut offset = 0;
    loop {
        let credential_rows = db.get_batch_credentials(offset, batch_size).unwrap();
        if credential_rows.is_empty() {
            break;
        }
        offset += batch_size;

        // decrypt and re encrypt using new password

        // decrypt and re encrypt using new password
        for row in credential_rows {
            let decrypted = Crypto::decrypt(&row.secret, &master_password, &row.nonce);
            let encrypted = Crypto::encrypt(&decrypted, &derived_new_key);
            db.update_credential_by_id(row.id, &encrypted.0, &encrypted.1)
                .unwrap();
        }
    }

    save_key_to_daemon(&derived_new_key);

    // display success message
    println!("Success: Master Password has been reset successfully.");
}
