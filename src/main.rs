mod cli;
mod core;
mod frameworks;
mod shell;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// zprof - Manage multiple zsh profiles with ease
#[derive(Debug, Parser)]
#[command(name = "zprof")]
#[command(about = "Manage multiple zsh profiles with ease", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new profile
    Create(cli::create::CreateArgs),
    /// Display the currently active profile
    Current(cli::current::CurrentArgs),
    /// Delete a profile
    Delete(cli::delete::DeleteArgs),
    /// Initialize zprof directory structure
    Init(cli::init::InitArgs),
    /// List all available zsh profiles
    List(cli::list::ListArgs),
    /// Restore original shell configuration (rollback zprof)
    Rollback(cli::rollback::RollbackArgs),
    /// Switch to a different profile
    Use(cli::use_cmd::UseArgs),
}

fn main() -> Result<()> {
    // Install panic hook to restore terminal on crashes
    tui::install_panic_hook();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create(args) => cli::create::execute(args),
        Commands::Current(args) => cli::current::execute(args),
        Commands::Delete(args) => cli::delete::execute(args),
        Commands::Init(args) => cli::init::execute(args),
        Commands::List(args) => cli::list::execute(args),
        Commands::Rollback(args) => cli::rollback::execute(args),
        Commands::Use(args) => cli::use_cmd::execute(args),
    }
}
