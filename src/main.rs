use clap::{Parser, Subcommand};
use jbrspass::arguments::*;
use anyhow::Result;

extern crate exitcode;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds password to list, if password for that user on that page doesn't already exist
    Add(AddArgs),

    /// Updates password for the username that already exists
    Update(UpdateArgs),

    /// Removes password for given username for given page
    Remove(RemoveArgs),

    /// Generates a random password for given username on a given page
    Generate(GenerateArgs),

    /// Gets password for given username for given page
    Get(GetArgs),
}

fn main() -> Result<()> {
    let args = Cli::parse();

    Ok(())
}