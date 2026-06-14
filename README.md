# shrsd

**A secure file shredder and data wipe utility written in Rust.**

When you normally delete a file or empty the Recycle Bin, the operating system only removes the file's index entry. The actual data remains on your disk until it is overwritten by new files, making it easily recoverable by data recovery software. 

`shrsd` acts as a clone of the GNU `shred` utility. It repeatedly overwrites file data in-place with cryptographic noise, flushes the OS disk cache, obfuscates the filename to wipe it from the Master File Table (MFT), and finally unlinks the file.

## ✨ Features
* **Secure Overwrite:** Overwrites file contents multiple times using cryptographically secure random data (`rand` crate).
* **MFT Obfuscation:** Renames files and directories iteratively with random characters before deletion to erase traces from the NTFS Master File Table.
* **Anti-Virus Bypass:** Implements smart back-off/retry loops to prevent "Access Denied" errors caused by Windows Defender locking files the instant they are modified.
* **Recursive Wiping:** Securely wipe entire directory trees.
* **Real-time Progress:** Beautiful progress bars and ETAs for large files (via `indicatif`).

---

## Important Note on SSDs
Due to a hardware feature called **Wear Leveling** managed by modern Solid State Drive (SSD) firmware, overwriting a file "in-place" often results in the drive writing the new data to a *new* physical block, leaving the old data intact until garbage collection runs. 

`shrsd` is highly effective against casual recovery tools and is perfect for mechanical HDDs, USB flash drives (without advanced controllers), and virtual machine disks. However, for absolute SSD security, **Full Disk Encryption** is a better approach.

---

## Installation

### Via Cargo (Git)
If you have [Rust and Cargo](https://rustup.rs/) installed, you can install the utility directly from this repository:

```bash
cargo install --git https://github.com/Robby-cell/shrsd.git
```

### Building from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/Robby-cell/shrsd.git
   cd shrsd
   ```
2. Build the release binary:
   ```bash
   cargo build --release
   ```
3. The executable will be available at `target/release/shrsd.exe`.

---

## Usage

By default, `shrsd` will **only overwrite** the file and leave the scrambled file on your disk. To actually delete the file after shredding, you must pass the `-u` (`--remove`) flag.

### Examples

**1. Shred and delete a single file, showing a progress bar:**
```bash
shrsd -u -v secret_document.pdf
```

**2. Shred a file 5 times, add a final pass of zeros, and delete it:**
```bash
shrsd -n 5 -z -u -v secret.txt
```

**3. Recursively shred an entire folder, forcing read-only files to be written:**
```bash
shrsd -r -u -f -v C:\path\to\confidential_folder
```

### Options

| Short | Long | Description |
| :---: | :--- | :--- |
| `-n`  | `--iterations <N>` | Number of times to overwrite the file (Default: 3) |
| `-z`  | `--zero`           | Add a final overwrite pass with pure zeros to hide shredding |
| `-u`  | `--remove`         | Truncate and remove the file/directory after overwriting |
| `-r`  | `--recursive`      | Recursively shred directories and their contents |
| `-f`  | `--force`          | Change file permissions to allow writing if read-only |
| `-v`  | `--verbose`        | Show real-time progress bars and operation logs |
| `-h`  | `--help`           | Print help information |
| `-V`  | `--version`        | Print version information |

---

## How it Works (Under the Hood)

1. **Permission Forcing:** If `-f` is used, the `FILE_ATTRIBUTE_READONLY` flag is stripped via the Windows API.
2. **Chunked Overwriting:** Generates buffers of cryptographic noise and writes them. It uses `File::sync_data()` (which triggers `FlushFileBuffers` in the Windows API) to ensure data is physically written to the disk platter/chips, bypassing the Windows RAM write-cache.
3. **Handle Release:** Safely drops the file handle.
4. **Obfuscation Loop:** Uses a Depth-First Search (DFS) for directories. When deleting, it pauses for `50ms` to let the Windows Search Indexer / Windows Defender release their automatic scan locks, then iteratively renames the file to random garbage (e.g., `secret.txt` -> `x9K2qA` -> `v8B`) to wipe the NTFS index.
5. **Unlinking:** Deletes the obfuscated file pointer from the disk.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
