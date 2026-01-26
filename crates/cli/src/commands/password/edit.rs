use vlt_crypto::Crypto;

use vlt_store::Db;

use crate::utils::{
    credential_utils::get_user_credentials_input,
    daemon_utils::get_master_key_from_user,
    selectable_utils::{get_selected_identifier, get_selected_service},
};

pub fn handler(db: &Db, flag_service: Option<String>, flag_identifier: Option<String>) {
    let key = get_master_key_from_user();

    let selected_service = match flag_service {
        None => get_selected_service(db, None),
        Some(value_srv) => value_srv,
    };

    let selected_identifier = match flag_identifier {
        None => get_selected_identifier(db, selected_service.to_string(), None),
        Some(value_ident) => value_ident,
    };

    if db
        .get_credential(&selected_service, &selected_identifier)
        .is_err()
    {
        println!(
            "Error: No existing credential for `{}` on `{}`. Use the add command to create it first.",
            selected_identifier, selected_service
        );
        return;
    }

    let secret_credential = get_user_credentials_input();
    let (encrypted, nonce) = Crypto::encrypt(&secret_credential, &key);
    db.edit_credential(&selected_service, &selected_identifier, &encrypted, &nonce)
        .unwrap();
    println!(
        "Success: Successfully edited `{}` credential for `{}`.",
        selected_service, selected_identifier
    );
}
