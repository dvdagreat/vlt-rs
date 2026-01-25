use core::Crypto;

use storage::Db;

use crate::utils::{
    credential_utils::get_user_credentials_input, daemon_utils::get_master_key_from_user,
};

pub fn handler(db: &Db, service: String, username: String) {
    let key = get_master_key_from_user();
    if db.get_credential(&service, &username).is_err() {
        println!(
            "Error: No existing credential for `{}` on `{}`. Use the add command to create it first.",
            username, service
        );
        return;
    }

    let secret_credential = get_user_credentials_input();
    let (encrypted, nonce) = Crypto::encrypt(&secret_credential, &key);
    db.edit_credential(&service, &username, &encrypted, &nonce)
        .unwrap();
    println!(
        "Success: Successfully edited `{}` credential for `{}`.",
        service, username
    );
}
