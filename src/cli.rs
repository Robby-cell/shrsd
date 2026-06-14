use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Securely overwrite files to prevent data recovery on Windows"
)]
pub struct Cli {
    /// Files to shred
    #[arg(required = true)]
    pub files: Vec<String>,

    /// Overwrite N times instead of the default (3)
    #[arg(short = 'n', long, default_value_t = 3)]
    pub iterations: u32,

    /// Add a final overwrite with zeros to hide shredding
    #[arg(short = 'z', long)]
    pub zero: bool,

    /// Truncate and remove file after overwriting
    #[arg(short = 'u', long)]
    pub remove: bool,

    /// Shred directories recursively
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Change permissions to allow writing if necessary
    #[arg(short = 'f', long)]
    pub force: bool,

    /// Show progress
    #[arg(short = 'v', long)]
    pub verbose: bool,
}
