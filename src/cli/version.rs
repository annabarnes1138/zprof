use anyhow::Result;
use clap::Args;

/// Display version information
#[derive(Debug, Args)]
pub struct VersionArgs {}

pub fn execute(_args: VersionArgs) -> Result<()> {
    println!("zprof {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
