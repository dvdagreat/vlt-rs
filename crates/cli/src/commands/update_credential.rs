use core::Crypto;

use storage::Db;

use crate::daemon_utils::get_master_key_from_user;

pub fn update_credential(db: &Db, service: String, username: String, secret: String) {
    let key = get_master_key_from_user();
    let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
    db.update_credential(&service, &username, &encrypted, &nonce)
        .unwrap();
    println!(
        "Success: Successfully updated `{}` credential for `{}`.",
        service, username
    );
}
