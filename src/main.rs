use clap::{Parser, Subcommand};
use rustpass::arguments::*;
use rustpass::init_config;
use rustpass::ops::*;
use anyhow::Result;

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

    /// Completely clears the password storage, use carefully
    Clear(ClearArgs),

    /// Change location of .pass_key directory containing encryption keys, default is home directory
    Config(ConfigArgs),

    /// Execute git command with given arguments, enables synchronization wit github
    Git(GitArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_config()?;

    match cli.command {
        Commands::Init(args) => init(&args.subfolder)?,
        Commands::Add(args) => add(&args.subfolder, &args.name, &args.password, &args.repeat_password,
                                                        args.copy)?,
        Commands::Update(args) => update(&args.subfolder, &args.name, &args.new_password, &args.repeat_password, args.copy)?,
        Commands::Remove(args) => remove(&args.subfolder, &args.name, args.copy)?,
        Commands::Generate(args) => generate(args.no_symbols, args.copy, args.no_print, args.saving.save, args.saving.new_save,
                                                           args.length, &args.subfolder, &args.name)?,
        Commands::Get(args) => get(&args.subfolder, &args.name, args.no_print, args.copy)?,
        Commands::List(args) => list(&args.subfolder)?,
        Commands::Clear(args) => clear(&args.subfolder)?,
        Commands::Config(args) => config(&args.path, args.get, args.reset)?,
        Commands::Git(args) => git(&args.args, args.clear)?,
    };

    Ok(())
}