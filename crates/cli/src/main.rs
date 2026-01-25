use clap::{Parser, Subcommand};
use storage::Db;

pub mod commands;
mod daemon_utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
