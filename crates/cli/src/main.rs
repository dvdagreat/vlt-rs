use clap::{Parser, Subcommand};
use storage::Db;

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
    Get {},

    /// Add a new password entry
    Add { srv: String, ident: String },

    /// Edit an existing password entry
    Edit { srv: String, ident: String },

    /// Remove a password entry
    Rm { srv: String, ident: String },
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

fn main() {
    let cli = Cli::parse();
    let db = Db::init("vault.db").expect("Failed to access the credential store");

    match cli.command {
        Commands::Cfg { action } => match action {
            ConfigCommands::Purge {} => commands::config::purge::handler(&db),
            ConfigCommands::ResetPass {} => commands::config::reset_pass::handler(&db),
        },
        Commands::Pass { action } => match action {
            PassCommands::Get {} => commands::password::get::handler(&db),
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
