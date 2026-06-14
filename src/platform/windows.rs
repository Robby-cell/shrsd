use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

use rand::RngExt;
use rand::distr::Alphanumeric;

/// Removes the read-only flag from a file on Windows
pub fn remove_readonly(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();

    if perms.readonly() {
        perms.set_readonly(false);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

// Renames the item multiple times to wipe the name, then deletes it
pub fn obfuscate_and_delete(path: &Path) -> io::Result<()> {
    let mut current_path = path.to_path_buf();

    // Check if we are deleting a directory or a file
    let is_dir = fs::symlink_metadata(&current_path)?.is_dir();

    let file_name_len = current_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.len())
        .unwrap_or(8);

    thread::sleep(Duration::from_millis(50));
    let mut rng = rand::rngs::ThreadRng::default();

    for len in (1..=file_name_len).rev() {
        let new_name: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();

        let new_path = current_path.with_file_name(&new_name);

        for _ in 0..5 {
            if fs::rename(&current_path, &new_path).is_ok() {
                current_path = new_path.clone();
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
    }

    // Finally, unlink from the filesystem based on the type
    for attempt in 0..5 {
        let result = if is_dir {
            fs::remove_dir(&current_path)
        } else {
            fs::remove_file(&current_path)
        };

        match result {
            Ok(_) => break,
            Err(e) if attempt == 4 => return Err(e),
            Err(_) => thread::sleep(Duration::from_millis(20)),
        }
    }

    Ok(())
}
