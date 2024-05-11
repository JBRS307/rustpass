use clap::{Parser, Subcommand};
use jbrspass::arguments::*;
use anyhow::Result;
use jbrspass::ops::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new subfolder
    Init(InitArgs),

    /// Adds password to list, if password for that user on that page doesn't already exist
    Add(AddArgs),

    /// Updates or adds new password
    Update(UpdateArgs),

    /// Removes password for given username for given page
    Remove(RemoveArgs),

    /// Generates a random password for given username on a given page
    Generate(GenerateArgs),

    /// Gets password for given username for given page
    Get(GetArgs),

    /// Lists pages for which passwords are saved
    List(ListArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => init(&args.path, &args.key),
        Commands::Add(args) => add(&args.page, &args.username, &args.password, &args.repeat_password,
                                                        args.copy),
        Commands::Update(args) => update(&args.page, &args.username, &args.new_password, &args.repeat_password,
                                                     args.copy_old, args.copy_new),
        Commands::Remove(args) => remove(&args.page, &args.username, args.copy),
        Commands::Generate(args) => generate(args.no_symbols, args.copy, args.no_print, args.saving.save, args.saving.new_save,
                                                           args.length, &args.page, &args.username),
        Commands::Get(args) => get(&args.page, &args.username, args.no_print, args.copy),
        Commands::List(args) => list(&args.path, args.recursive),
    };

    Ok(())
}