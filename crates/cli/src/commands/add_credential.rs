use core::Crypto;

use storage::Db;

use crate::daemon_utils::get_master_key_from_user;

pub fn add_credential(db: &Db, service: String, username: String, secret: String) {
    let key = get_master_key_from_user();
    let (encrypted, nonce) = Crypto::encrypt(&secret, &key);
    db.add_credential(&service, &username, &encrypted, &nonce)
        .unwrap();
    println!(
        "Success: Successfully saved `{}` credential for `{}`.",
        service, username
    );
}
