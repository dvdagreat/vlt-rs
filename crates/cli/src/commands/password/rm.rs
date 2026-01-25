use storage::Db;

use crate::daemon_utils::get_master_key_from_user;

pub fn handler(db: &Db, service: String, identifier: String) {
    let key = get_master_key_from_user();

    if key.is_empty() {
        println!("Error: Master key is required to remove credentials.");
        return;
    }

    if db.get_credential(&service, &identifier).is_err() {
        println!(
            "Error: No existing credential for `{}` on `{}`.",
            identifier, service
        );
        return;
    }

    db.delete_credential(&service, &identifier).unwrap();
    println!(
        "Success: Successfully removed `{}` credential for `{}`.",
        service, identifier
    );
}
