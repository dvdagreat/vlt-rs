use dialoguer::{Select, theme::ColorfulTheme};
use storage::Db;

pub fn get_selected_service(db: &Db, add_options: Option<Vec<String>>) -> String {
    let mut options = db
        .get_service_list()
        .expect("Cannot Fetch Selectable options");

    match add_options {
        Some(add_options) => options.append(&mut add_options.clone()),
        None => {}
    }

    if options.len() == 0 {
        return "".to_string();
    }

    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("which service will you select?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Cannot prepare a select list for you at the moment");

    // println!("You selected: {}", options[selection]);
    options[selected_idx].to_owned()
}

pub fn get_selected_identifier(
    db: &Db,
    service_name: String,
    add_options: Option<Vec<String>>,
) -> String {
    let mut options = db
        .get_identifier_list_by_service_name(service_name)
        .expect("Cannot Fetch Selectable options");

    match add_options {
        Some(add_options) => options.append(&mut add_options.clone()),
        None => {}
    }

    if options.len() == 0 {
        return "".to_string();
    }

    let selected_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("which identifier will you select?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Cannot prepare a select list for you at the moment");

    // println!("You selected: {}", options[selection]);
    options[selected_idx].to_owned()
}
