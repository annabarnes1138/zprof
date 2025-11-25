mod archive;
mod backup;
mod cli;
mod core;
mod frameworks;
mod git;
mod presets;
mod prompts;
mod shell;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// zprof - Manage multiple zsh profiles with ease
///
/// Available via CLI (this tool) or GUI (run 'zprof gui' to launch graphical interface)
#[derive(Debug, Parser)]
#[command(name = "zprof")]
#[command(version)]
#[command(about = "Manage multiple zsh profiles with ease")]
#[command(long_about = "zprof allows you to manage multiple isolated zsh profiles.\n\
Switch between configurations instantly, experiment safely, and share profiles.\n\n\
Available via CLI (this tool) or GUI (run 'zprof gui' to launch graphical interface).")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List available frameworks, plugins, and themes
    Available(cli::available::AvailableArgs),
    /// Create a new profile
    Create(cli::create::CreateArgs),
    /// Display the currently active profile
    Current(cli::current::CurrentArgs),
    /// Delete a profile
    Delete(cli::delete::DeleteArgs),
    /// Edit a profile's TOML manifest
    Edit(cli::edit::EditArgs),
    /// Export a profile to a .zprof archive
    Export(cli::export::ExportArgs),
    /// Launch the graphical user interface
    #[cfg(feature = "gui")]
    Gui(cli::gui::GuiArgs),
    /// Import a profile from a .zprof archive
    Import(cli::import::ImportArgs),
    /// Initialize zprof directory structure
    Init(cli::init::InitArgs),
    /// List all available zsh profiles
    List(cli::list::ListArgs),
    /// Regenerate shell configuration files from profile.toml
    Regenerate(cli::regenerate::RegenerateArgs),
    /// Restore original shell configuration (rollback zprof)
    Rollback(cli::rollback::RollbackArgs),
    /// Show detailed information about a profile
    Show(cli::show::ShowArgs),
    /// Switch to a different profile
    Use(cli::use_cmd::UseArgs),
    /// Display version information
    Version(cli::version::VersionArgs),
}

fn main() -> Result<()> {
    // Install panic hook to restore terminal on crashes
    tui::install_panic_hook();

    let cli = Cli::parse();

    match cli.command {
        Commands::Available(args) => cli::available::execute(args),
        Commands::Create(args) => cli::create::execute(args),
        Commands::Current(args) => cli::current::execute(args),
        Commands::Delete(args) => cli::delete::execute(args),
        Commands::Edit(args) => cli::edit::execute(args),
        Commands::Export(args) => cli::export::execute(args),
        #[cfg(feature = "gui")]
        Commands::Gui(args) => cli::gui::execute(args),
        Commands::Import(args) => cli::import::execute(args),
        Commands::Init(args) => cli::init::execute(args),
        Commands::List(args) => cli::list::execute(args),
        Commands::Regenerate(args) => cli::regenerate::execute(args),
        Commands::Rollback(args) => cli::rollback::execute(args),
        Commands::Show(args) => cli::show::execute(args),
        Commands::Use(args) => cli::use_cmd::execute(args),
        Commands::Version(args) => cli::version::execute(args),
    }
}
