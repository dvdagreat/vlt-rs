use clap::{Parser, Subcommand};
use storage::Db;

use crate::commands::add_credential::add_credential;
use crate::commands::get_credential::get_credential;
use crate::commands::nuke_credentials::nuke_credentials;
use crate::commands::update_credential::update_credential;

mod commands;
mod daemon_utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save a new credential
    Add {
        service: String,
        username: String,
        secret: String,
    },
    /// Copy credential to clipboard
    Get { service: String, identifier: String },
    /// Update an existing credential
    Update {
        service: String,
        identifier: String,
        secret: String,
    },
    /// Wipe all data and forget master password
    Nuke,
}

fn main() {
    let cli = Cli::parse();
    let db = Db::init("vault.db").expect("Failed to access the credential store");

    match cli.command {
        Commands::Add {
            service,
            username,
            secret,
        } => add_credential(&db, service, username, secret),
        Commands::Get {
            service,
            identifier,
        } => get_credential(&db, service, identifier),
        Commands::Update {
            service,
            identifier,
            secret,
        } => update_credential(&db, service, identifier, secret),
        Commands::Nuke => nuke_credentials(&db),
    }
}
