mod cli;
mod core;
mod frameworks;

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
    /// Initialize zprof directory structure
    Init(cli::init::InitArgs),
    /// List all available zsh profiles
    List(cli::list::ListArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create(args) => cli::create::execute(args),
        Commands::Current(args) => cli::current::execute(args),
        Commands::Init(args) => cli::init::execute(args),
        Commands::List(args) => cli::list::execute(args),
    }
}
