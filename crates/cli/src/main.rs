use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use vlt_store::Db;

pub mod commands;
pub mod utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage your master password
    Cfg {
        #[command(subcommand)]
        action: ConfigCommands,
    },

    /// Manage your stored passwords
    Pass {
        #[command(subcommand)]
        action: PassCommands,
    },

    /// Manage your identifiers (future, not implemented yet)
    Ident {
        #[command(subcommand)]
        action: IdentCommands,
    },

    /// Manage your services (future, not implemented yet)
    Srv {
        #[command(subcommand)]
        action: ServiceCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Purge all stored data
    Purge {},

    /// Reset your master password
    ResetPass {},
}

#[derive(Subcommand)]
enum PassCommands {
    /// Retrieve a stored password and store in the clipboard
    Get {
        #[arg(short = 'S', long = "srv")]
        srv: Option<String>,

        #[arg(short = 'I', long = "ident")]
        ident: Option<String>,
    },

    /// Add a new password entry
    Add {
        #[arg(short = 'S', long = "srv")]
        srv: Option<String>,

        #[arg(short = 'I', long = "ident")]
        ident: Option<String>,
    },

    /// Edit an existing password entry
    Edit {
        #[arg(short = 'S', long = "srv")]
        srv: Option<String>,

        #[arg(short = 'I', long = "ident")]
        ident: Option<String>,
    },

    /// Remove a password entry
    Rm {
        #[arg(short = 'S', long = "srv")]
        srv: Option<String>,

        #[arg(short = 'I', long = "ident")]
        ident: Option<String>,
    },
}

#[derive(Subcommand)]
enum IdentCommands {
    /// List all identifiers
    List {},
    /// Set an identifier
    Set {},
    /// Remove an identifier
    Rm {},
}

#[derive(Subcommand)]
enum ServiceCommands {
    /// List all services
    List {},
    /// Set a service
    Set {},
    /// Remove a service
    Rm {},
}

fn get_store_path() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".vlt");
    fs::create_dir_all(&dir).unwrap();

    dir.join("vault.db")
}

fn main() {
    let cli = Cli::parse();
    let db_path = get_store_path();
    let db = Db::init(db_path.to_str().unwrap()).expect("Failed to access the credential store");

    match cli.command {
        Commands::Cfg { action } => match action {
            ConfigCommands::Purge {} => commands::config::purge::handler(&db),
            ConfigCommands::ResetPass {} => commands::config::reset_pass::handler(&db),
        },
        Commands::Pass { action } => match action {
            PassCommands::Get { srv, ident } => commands::password::get::handler(&db, srv, ident),
            PassCommands::Add { srv, ident } => commands::password::add::handler(&db, srv, ident),
            PassCommands::Edit { srv, ident } => commands::password::edit::handler(&db, srv, ident),
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
