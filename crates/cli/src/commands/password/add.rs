use vlt_crypto::Crypto;

use vlt_store::Db;

use crate::utils::{
    credential_utils::get_user_credentials_input,
    daemon_utils::get_master_key_from_user,
    selectable_utils::{
        get_selected_identifier_for_add_password, get_selected_service_for_add_password,
    },
};
pub fn handler(db: &Db, flag_service: Option<String>, flag_identifier: Option<String>) {
    let key = get_master_key_from_user();

    let selected_service = match flag_service {
        None => get_selected_service_for_add_password(db),
        Some(value_srv) => value_srv,
    };

    if selected_service == "" {
        println!("Notice: Either there are no services stored or you didn't select a service");
        return;
    }

    let selected_identifier = match flag_identifier {
        None => get_selected_identifier_for_add_password(),
        Some(value_ident) => value_ident,
    };

    if selected_identifier == "" {
        println!(
            "Notice: Either there are no identifiers stored or you didn't select an identifier"
        );
        return;
    }

    if db
        .get_credential(&selected_service, &selected_identifier)
        .is_ok()
    {
        println!(
            "Error: Credential for `{}` on `{}` already exists. Use the edit command to modify it.",
            selected_identifier, selected_service
        );
        return;
    }

    let secret_credential = get_user_credentials_input();
    if secret_credential == "" {
        println!("Error: Credentials cannot be empty");
        return;
    }
    let (encrypted, nonce) = Crypto::encrypt(&secret_credential, &key);
    db.add_credential(&selected_service, &selected_identifier, &encrypted, &nonce)
        .unwrap();
    println!(
        "Success: Successfully saved `{}` credential for `{}`.",
        selected_service, selected_identifier
    );
}
