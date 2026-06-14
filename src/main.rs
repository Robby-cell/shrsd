mod cli;
mod passes;
mod platform;
mod shredder;

use std::path::Path;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    let mut success = true;

    for file_path in &cli.files {
        // Pass Path::new instead of the raw string, and call process_target
        if let Err(e) = shredder::process_target(Path::new(file_path), &cli) {
            eprintln!("shred: {}: failed to shred - {}", file_path, e);
            success = false;
        }
    }

    if !success {
        std::process::exit(1);
    }
}
