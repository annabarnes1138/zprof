mod cli;
mod core;

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
        Commands::Current(args) => cli::current::execute(args),
        Commands::Init(args) => cli::init::execute(args),
        Commands::List(args) => cli::list::execute(args),
    }
}
