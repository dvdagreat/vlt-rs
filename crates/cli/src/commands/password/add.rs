use core::Crypto;

use storage::Db;

use crate::daemon_utils::get_master_key_from_user;
pub fn handler(db: &Db, service: String, username: String, secret: String) {
    let key = get_master_key_from_user();
    if db.get_credential(&service, &username).is_ok() {
        println!(
            "Error: Credential for `{}` on `{}` already exists. Use the edit command to modify it.",
            username, service
        );
        return;
    }

    let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
    db.add_credential(&service, &username, &encrypted, &nonce)
        .unwrap();
    println!(
        "Success: Successfully saved `{}` credential for `{}`.",
        service, username
    );
}
