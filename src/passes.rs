use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};

use indicatif::ProgressBar;
use rand::Rng;

const BUFFER_SIZE: usize = 64 * 1024; // 64 KB chunk size

/// Overwrites the file with cryptographically random data
pub fn write_random_pass(file: &mut File, size: u64, pb: &ProgressBar) -> io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut bytes_written: u64 = 0;
    let mut rng = rand::rngs::ThreadRng::default();

    while bytes_written < size {
        rng.fill_bytes(&mut buffer);

        let to_write = std::cmp::min(BUFFER_SIZE as u64, size - bytes_written) as usize;
        file.write_all(&buffer[..to_write])?;

        bytes_written += to_write as u64;
        pb.inc(to_write as u64); // Update the progress bar
    }

    file.sync_data()?;
    Ok(())
}

/// Overwrites the file with purely zeros
pub fn write_zero_pass(file: &mut File, size: u64, pb: &ProgressBar) -> io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    let buffer = vec![0u8; BUFFER_SIZE];
    let mut bytes_written: u64 = 0;

    while bytes_written < size {
        let to_write = std::cmp::min(BUFFER_SIZE as u64, size - bytes_written) as usize;
        file.write_all(&buffer[..to_write])?;

        bytes_written += to_write as u64;
        pb.inc(to_write as u64); // Update the progress bar
    }

    file.sync_data()?;
    Ok(())
}
