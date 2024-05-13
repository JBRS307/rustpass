use clap::Args;
use core::arch;
use std::path::PathBuf;

#[derive(Args)]
pub struct InitArgs {
    /// Subfolder to be initialized
    #[arg(short, long)]
    pub subfolder: Option<PathBuf>,
}

#[derive(Args)]
pub struct AddArgs {
    /// Path to subfolder, i.e. subfolder/page/username
    #[arg(short, long)]
    pub subfolder: Option<PathBuf>,

    /// Name by which password should be indentified
    pub name: String,

    /// Password you want to add
    pub password: String,

    /// Repetition of your password to avoid typos
    pub repeat_password: String,

    /// Copies password to clipboard
    #[arg(short, long)]
    pub copy: bool,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Path to subfolder, i.e. subfolder/page/username
    #[arg(short, long)]
    pub subfolder: Option<PathBuf>,

    /// Name by which password is identifies
    pub name: String,

    /// Your new password
    pub new_password: String,

    /// Repetition of your new password to avoid typos
    pub repeat_password: String,

    /// Copies old password to clipboard
    #[arg(long)]
    pub copy_old: bool,

    /// Copies new password to clipboard
    #[arg(short, long)]
    pub copy_new: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Path to subfolder, i.e. subfolder/page/username
    #[arg(short, long)]
    pub subfolder: Option<PathBuf>,

    /// Name by which password is identifies
    pub name: String,

    /// Copies removed password to clipboard
    #[arg(short, long)]
    pub copy: bool,
}

#[derive(Args)]
pub struct GenerateArgs {
    /// Doesn't generate special symbols in your password
    #[arg(long)]
    pub no_symbols: bool,

    /// Copies newly generated password to clipboard
    #[arg(short, long)]
    pub copy: bool,

    /// Doesn't print newly generated password if specified
    #[arg(short, long)]
    pub no_print: bool,

    #[command(flatten)]
    pub saving: GenSaves,

    /// Desired length of a password
    pub length: u32,

    /// Path to subfolder, i.e. subfolder/page/username
    #[clap(required_if_eq_any([("save", "true"), ("new_save", "true")]))]
    pub subfolder: Option<PathBuf>,

    /// Name by which password should be identified
    #[clap(required_if_eq_any([("save", "true"), ("new_save", "true")]))]
    pub name: Option<String>,
}

#[derive(Args)]
#[group(multiple = false)]
pub struct GenSaves {
    /// Saves generated password for given page and username
    #[arg(short, long)]
    pub save: bool,

    /// Saves generated password for given page and username ONLY if it doesn't already exist
    #[arg(long)]
    pub new_save: bool
}

#[derive(Args)]
pub struct GetArgs {
    /// Path to password, i.e. subfolder/page/username
    #[arg(short, long)]
    pub subfolder: Option<PathBuf>,

    /// Name by which password should be identified
    pub name: String,

    /// Doesn't print the password
    #[arg(short, long, requires = "copy")]
    pub no_print: bool,

    /// Copies password to clipboard
    #[arg(short, long)]
    pub copy: bool
}

#[derive(Args)]
pub struct ListArgs {
    /// Subfolder which you want to be listed, if not specified, main folder is listed
    pub subfolder: Option<PathBuf>,
}

#[derive(Args)]
pub struct ClearArgs {
    /// Path to the subfolder that should be deleted, if not given, removes the entire storage directory
    #[arg(short, long)]
    pub subfolder: Option<PathBuf>
}

#[derive(Args)]
pub struct ConfigArgs {
    /// Path to new key location
    #[clap(required_unless_present_any(["get", "reset"]))]
    pub path: Option<PathBuf>,

    /// Print current password key location
    #[arg(short, long)]
    pub get: bool,

    /// Resets config to default
    #[arg(short, long)]
    pub reset: bool,
}