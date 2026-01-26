use vlt_store::Db;

use crate::utils::{
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

    if key.is_empty() {
        println!("Error: Master key is required to remove credentials.");
        return;
    }

    if db
        .get_credential(&selected_service, &selected_identifier)
        .is_err()
    {
        println!(
            "Error: No existing credential for `{}` on `{}`.",
            selected_identifier, selected_service
        );
        return;
    }

    db.delete_credential(&selected_service, &selected_identifier)
        .unwrap();
    println!(
        "Success: Successfully removed `{}` credential for `{}`.",
        selected_service, selected_identifier
    );
}
