use std::fs::{self, OpenOptions};
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::Cli;
use crate::passes;
use crate::platform;

/// The entry point for processing a path (file or directory)
pub fn process_target(path: &Path, config: &Cli) -> std::io::Result<()> {
    // We use symlink_metadata so we don't accidentally follow shortcuts
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return Ok(()), // Skip if file doesn't exist or is inaccessible
    };

    // 1. Safety feature: NEVER follow symlinks.
    if meta.is_symlink() {
        if config.verbose {
            println!("shred: skipping symlink {}", path.display());
        }
        return Ok(());
    }

    // 2. Handle Directories
    if meta.is_dir() {
        if config.recursive {
            // Read folder contents and recursively process them
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                process_target(&entry.path(), config)?; // Recursive call
            }

            // Once the directory is empty, remove it if requested
            if config.remove {
                if config.force {
                    platform::remove_readonly(path)?; // Ensure the dir isn't read-only
                }
                if config.verbose {
                    println!("shred: removing directory {}", path.display());
                }
                platform::obfuscate_and_delete(path)?;
                if config.verbose {
                    println!("shred: removed directory");
                }
            }
        } else {
            eprintln!(
                "shred: {}: is a directory (use -r to shred recursively)",
                path.display()
            );
        }
        return Ok(());
    }

    // 3. Handle standard files
    process_file(path, config)
}

/// The actual file overwriting logic (your existing code, slightly updated to accept &Path)
fn process_file(path: &Path, config: &Cli) -> std::io::Result<()> {
    if config.force {
        platform::remove_readonly(path)?;
    }

    let mut file = OpenOptions::new().write(true).open(path)?;
    let size = file.metadata()?.len();

    let pb = if config.verbose {
        let pb = ProgressBar::new(size);
        pb.set_style(
            ProgressStyle::with_template(
                "{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
            )
            .unwrap()
            .progress_chars("#>-")
        );
        pb
    } else {
        ProgressBar::hidden()
    };

    for pass in 1..=config.iterations {
        if config.verbose {
            pb.set_position(0);
            pb.set_message(format!(
                "shredding: {} (pass {}/{})",
                path.file_name().unwrap().to_string_lossy(),
                pass,
                config.iterations
            ));
        }
        passes::write_random_pass(&mut file, size, &pb)?;
    }

    if config.zero {
        if config.verbose {
            pb.set_position(0);
            pb.set_message(format!(
                "shredding: {} (pass zero)",
                path.file_name().unwrap().to_string_lossy()
            ));
        }
        passes::write_zero_pass(&mut file, size, &pb)?;
    }

    if config.verbose {
        pb.finish_with_message(format!(
            "{} overwritten successfully.",
            path.file_name().unwrap().to_string_lossy()
        ));
    }

    drop(file);

    if config.remove {
        if config.verbose {
            println!("shred: removing {}", path.display());
        }
        platform::obfuscate_and_delete(path)?;
    }

    Ok(())
}
