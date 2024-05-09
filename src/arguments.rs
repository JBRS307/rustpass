use clap::Args;

#[derive(Args)]
pub struct AddArgs {
    /// Page for which the password is stored, it doesn't have to be a URL
    pub page: String,

    /// Username for which password should be saved
    pub username: String,

    /// Password you want to add
    pub password: String,

    /// Repetition of your password to avoid typos
    pub repeat_password: String,

    /// Copies password to clipboard
    #[arg(short, long)]
    copy: bool,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Page for which the password is stored, it doesn't have to be a URL
    pub page: String,

    /// Username for which password should be updated
    pub username: String,

    /// Your new password
    pub new_password: String,

    /// Repetition of your new password to avoid typos
    pub repeat_password: String,

    /// Copies old password to clipboard
    #[arg(long)]
    copy_old: bool,

    /// Copies new password to clipboard
    #[arg(short, long)]
    copy_new: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Page for which the password is stored, it doesn't have to be a URL
    pub page: String,

    /// Username for which password should be removed
    pub username: String,

    /// Copies removed password to clipboard
    #[arg(short, long)]
    copy: bool,
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

    /// Page to save the password for
    #[clap(required_if_eq_any([("save", "true"), ("new_save", "true")]))]
    pub page: Option<String>,

    /// Username for which password should be saved
    #[clap(required_if_eq_any([("save", "true"), ("new_save", "true")]))]
    pub username: Option<String>
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
    /// Page for which the password is stored, it doesn't have to be a URL
    pub page: String,

    /// Username for wich password should be read
    pub username: String,

    /// Doesn't print the password
    #[arg(short, long, requires = "copy")]
    pub no_print: bool,

    /// Copies password to clipboard
    #[arg(short, long)]
    pub copy: bool
}
