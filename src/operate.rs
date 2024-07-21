use std::path::{Path, PathBuf};
use std::{fs, io};

fn create_symlink(source: &Path, destination: &Path) -> io::Result<()> {
    if source.is_file() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        if destination.exists() {
            fs::remove_file(destination)?;
        }
        #[cfg(target_os = "windows")]
        std::os::windows::fs::symlink_file(source, destination)?;
        #[cfg(not(target_os = "windows"))]
        std::os::unix::fs::symlink(source, destination)?;
    } else if source.is_dir() {
        if destination.exists() {
            fs::remove_dir_all(destination)?;
        }
        #[cfg(target_os = "windows")]
        std::os::windows::fs::symlink_file(source, destination)?;
        #[cfg(not(target_os = "windows"))]
        std::os::unix::fs::symlink(source, destination)?;
    }
    Ok(())
}

/// stow - this is the operation behind the stow command. It will accept a
/// source: PathBuf and a destination: PathBuf
pub fn stow(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    if source.is_dir() {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = destination.join(entry.file_name());
            if entry_path.is_file() || entry_path.is_dir() {
                create_symlink(&entry_path, &dest_path)?;
            }
        }
    } else {
        eprintln!("Error: Source path is not a directory");
    }
    Ok(())
}
