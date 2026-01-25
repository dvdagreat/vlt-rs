use clap::{Parser, Subcommand};
use storage::Db;

use crate::commands::add_credential::add_credential;
use crate::commands::get_credential::get_credential;
use crate::commands::nuke_credentials::nuke_credentials;
use crate::commands::update_credential::update_credential;

pub mod commands;
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
    /// Config subcommands
    Cfg {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    Pass {
        #[command(subcommand)]
        action: PassCommands,
    },
    Ident {
        #[command(subcommand)]
        action: IdentCommands,
    },
    Srv {
        #[command(subcommand)]
        action: ServiceCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    Purge {},
    ResetPass {},
}

#[derive(Subcommand)]
enum PassCommands {
    Get {
        srv: String,
        ident: String,
    },
    Add {
        srv: String,
        ident: String,
        secret: String,
    },
    Edit {
        srv: String,
        ident: String,
        secret: String,
    },
    Rm {
        srv: String,
        ident: String,
    },
}

#[derive(Subcommand)]
enum IdentCommands {
    List {},
    Set {},
    Rm {},
}

#[derive(Subcommand)]
enum ServiceCommands {
    List {},
    Set {},
    Rm {},
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
        Commands::Cfg { action } => match action {
            ConfigCommands::Purge {} => commands::config::purge::handler(&db),
            ConfigCommands::ResetPass {} => commands::config::reset_pass::handler(&db),
        },
        Commands::Pass { action } => match action {
            PassCommands::Get { srv, ident } => commands::password::get::handler(&db, srv, ident),
            PassCommands::Add { srv, ident, secret } => {
                commands::password::add::handler(&db, srv, ident, secret)
            }
            PassCommands::Edit { srv, ident, secret } => {
                commands::password::edit::handler(&db, srv, ident, secret)
            }
            PassCommands::Rm { srv, ident } => commands::password::rm::handler(&db, srv, ident),
        },
        Commands::Ident { action } => match action {
            IdentCommands::List {} => commands::identifier::list_command(),
            IdentCommands::Set {} => commands::identifier::set_command(),
            IdentCommands::Rm {} => commands::identifier::rm_command(),
        },
        Commands::Srv { action } => match action {
            ServiceCommands::List {} => commands::service::list_command(),
            ServiceCommands::Set {} => commands::service::set_command(),
            ServiceCommands::Rm {} => commands::service::rm_command(),
        },
    };
}
