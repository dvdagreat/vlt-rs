use dialoguer::{Select, theme::ColorfulTheme};
use storage::Db;

pub fn get_selected_service(db: &Db) -> String {
    let options = db
        .get_service_list()
        .expect("Cannot Fetch Selectable options");

    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("which service will you select?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Cannot prepare a select list for you at the moment");

    // println!("You selected: {}", options[selection]);
    options[selected_idx].to_owned()
}

pub fn get_selected_identifier(db: &Db, service_name: String) -> String {
    let options = db
        .get_identifier_list_by_service_name(service_name)
        .expect("Cannot Fetch Selectable options");

    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("which identifier will you select?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Cannot prepare a select list for you at the moment");

    // println!("You selected: {}", options[selection]);
    options[selected_idx].to_owned()
}
