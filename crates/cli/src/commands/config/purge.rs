use std::io::{self, Write};

use vlt_store::Db;

pub fn handler(db: &Db) {
    print!("DANGER ZONE: Are you ABSOLUTELY sure you want to erase EVERYTHING? (y/N): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "y" {
        db.delete_all_data().unwrap();
        println!("Success: All passwords dropped from the database.");
    }
}
