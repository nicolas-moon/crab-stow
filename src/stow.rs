use log::{info, warn};
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use crate::cli::StowConfig;

/// Main stow struct to manage symlink operations
pub struct CrabStow {
    config: StowConfig,
}

impl CrabStow {
    /// create a new CrabStow instance
    pub fn new(config: StowConfig) -> Self {
        Self { config }
    }

    /// Stow or unstow based on configuration
    pub fn execute(&self) -> Result<()> {
        if self.config.restow {
            self.unstow()?;
        }

        if self.config.unstow {
            self.unstow()
        } else {
            self.stow()
        }
    }

    /// stow a package by creating symlinks
    pub fn stow(&self) -> Result<()> {
        let package_path = self.config.stow_dir.join(&self.config.package_name);
        let destination_path = self.config.target_dir.join(&self.config.package_name);

        if !destination_path.exists() {
            if self.config.simulate {
                println!("Would create package directory: {:?}", destination_path);
            } else {
                fs::create_dir_all(&destination_path)?;
                if self.config.verbose > 0 {
                    info!("Created package directory: {:?}", destination_path);
                }
            }
        }

        if self.config.verbose > 0 {
            info!("Stowing package: {}", self.config.package_name);
        }

        self.traverse_and_link(&package_path, &destination_path)
    }

    /// Unstow a package by removing symlinks
    pub fn unstow(&self) -> Result<()> {
        let package_path = self.config.stow_dir.join(&self.config.package_name);
        let destination_path = self.config.target_dir.join(&self.config.package_name);

        if !package_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Package {} not found", self.config.package_name),
            ));
        }

        if self.config.verbose > 0 {
            info!("Unstowing package: {}", self.config.package_name);
        }

        let result = self.traverse_and_unlink(&package_path, &destination_path);

        // Clean up the package directory itself if unstowing left it empty.
        if result.is_ok() && self.is_dir_empty(&destination_path) {
            self.remove_path(&destination_path, "directory")?;
        }

        result
    }

    /// Recursively traverse directories and create symlinks
    fn traverse_and_link(&self, source: &Path, target: &Path) -> Result<()> {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let source_path = entry.path();
            let relative_path = source_path
                .strip_prefix(source)
                .expect("paths returned by read_dir are always under the source directory");
            let target_path = target.join(relative_path);

            if file_type.is_dir() {
                // create dir if does not exist
                if !target_path.exists() {
                    if self.config.simulate {
                        println!("Would create directory: {:?}", target_path);
                    } else {
                        fs::create_dir_all(&target_path)?;
                    }
                }

                // recursively link subdir's
                self.traverse_and_link(&source_path, &target_path)?;
            } else if file_type.is_file() {
                // create symlink for file
                if self.config.simulate {
                    println!("Would symlink: {:?} -> {:?}", source_path, target_path);
                } else {
                    if target_path.exists() {
                        if self.config.verbose > 0 {
                            warn!("Skipping existing path: {:?}", target_path);
                        }
                        continue;
                    }

                    #[cfg(unix)]
                    {
                        std::os::unix::fs::symlink(&source_path, &target_path)?;
                        if self.config.verbose > 0 {
                            info!("Created symlink: {:?} -> {:?}", source_path, target_path);
                        }
                    }

                    #[cfg(windows)]
                    {
                        // windows symlink creation (might require admin privs)
                        std::os::windows::fs::symlink_file(&source_path, &target_path)?;
                        if self.config.verbose > 0 {
                            info!("Created symlink: {:?} -> {:?}", source_path, target_path);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Recursively remove symlinks for a package
    fn traverse_and_unlink(&self, source: &Path, target: &Path) -> Result<()> {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let source_path = entry.path();
            let relative_path = source_path
                .strip_prefix(source)
                .expect("paths returned by read_dir are always under the source directory");
            let target_path = target.join(relative_path);

            if file_type.is_dir() {
                if target_path.is_symlink() {
                    self.remove_path(&target_path, "directory")?;
                } else if target_path.exists() {
                    // recurse into the directory, then remove it if unstowing
                    // left it empty
                    self.traverse_and_unlink(&source_path, &target_path)?;
                    if self.is_dir_empty(&target_path) {
                        self.remove_path(&target_path, "directory")?;
                    }
                }
            } else if file_type.is_file() {
                // remove symlinks
                if target_path.is_symlink() {
                    self.remove_path(&target_path, "symlink")?;
                }
            }
        }
        Ok(())
    }

    /// Remove a path, or report that it would be removed in simulate mode
    fn remove_path(&self, path: &Path, kind: &str) -> Result<()> {
        if self.config.simulate {
            println!("Would remove {kind}: {:?}", path);
            return Ok(());
        }

        if path.is_dir() {
            fs::remove_dir(path)?;
        } else {
            fs::remove_file(path)?;
        }
        if self.config.verbose > 0 {
            info!("Removed {kind}: {:?}", path);
        }
        Ok(())
    }

    /// check if a directory is empty
    fn is_dir_empty(&self, path: &Path) -> bool {
        path.read_dir()
            .is_ok_and(|mut entries| entries.next().is_none())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::StowConfig;
    use std::path::PathBuf;

    fn minimal_config(target_dir: &Path, stow_dir: &Path) -> StowConfig {
        StowConfig {
            target_dir: PathBuf::from(target_dir),
            stow_dir: PathBuf::from(stow_dir),
            package_name: "dotfiles".to_string(),
            simulate: false,
            verbose: 0,
            no_folding: false,
            adopt: false,
            restow: false,
            unstow: false,
        }
    }

    #[test]
    fn is_dir_empty_detects_empty_and_nonempty_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        let crab = CrabStow::new(minimal_config(tmp.path(), tmp.path()));

        assert!(crab.is_dir_empty(tmp.path()));

        fs::write(tmp.path().join("file"), "x").unwrap();
        assert!(!crab.is_dir_empty(tmp.path()));

        // a non-existent directory is not "empty" (it cannot be removed as a dir)
        assert!(!crab.is_dir_empty(&tmp.path().join("does-not-exist")));
    }
}
