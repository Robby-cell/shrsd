use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use rand::RngExt;
use rand::distr::Alphanumeric;

/// Removes the read-only flag from a file on Unix (adds owner write permission)
pub fn remove_readonly(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();

    // On Unix, this modifies the mode to add the owner write bit (e.g., chmod u+w)
    if perms.readonly() {
        perms.set_readonly(false);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

/// Renames the item multiple times to wipe the name from the filesystem index, then unlinks it
pub fn obfuscate_and_delete(path: &Path) -> io::Result<()> {
    let mut current_path = path.to_path_buf();

    // Check if we are deleting a directory or a file
    let is_dir = fs::symlink_metadata(&current_path)?.is_dir();

    let file_name_len = current_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.len())
        .unwrap_or(8);

    let mut rng = rand::rngs::ThreadRng::default();

    // Iteratively rename the file. E.g., secret.txt -> x9K2qA -> v8B -> ... -> X
    for len in (1..=file_name_len).rev() {
        let new_name: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();

        let new_path = current_path.with_file_name(&new_name);

        // Unix doesn't have AV locks, so a single, immediate rename attempt is perfectly safe
        if fs::rename(&current_path, &new_path).is_ok() {
            current_path = new_path;
        }
    }

    // Unlink the file or directory directly
    if is_dir {
        fs::remove_dir(&current_path)?;
    } else {
        fs::remove_file(&current_path)?;
    }

    Ok(())
}
