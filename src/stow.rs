use log::{info, warn};
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use crate::config::StowConfig;

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
                if self.config.verbose {
                    info!("Created package directory: {:?}", destination_path);
                }
            }
        }

        if self.config.verbose {
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
                format!("Pakcage {} not found", self.config.package_name),
            ));
        }

        if self.config.verbose {
            info!("Unstowing package: {}", self.config.package_name);
        }

        self.traverse_and_unlink(&package_path, &destination_path)
    }

    ///Recursively traverse directories and create symlinks
    fn traverse_and_link(&self, source: &Path, target: &Path) -> Result<()> {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let source_path = entry.path();
            let relative_path = source_path.strip_prefix(source).unwrap();
            let target_path = target.join(relative_path);

            if file_type.is_dir() {
                // create dir if does not exist
                if !target_path.exists() {
                    if self.config.simulate {
                        println!("Would create directory: {:?}", target_path);
                    } else {
                        let _ = fs::create_dir_all(&target_path);
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
                        if self.config.verbose {
                            warn!("Skipping existing path: {:?}", target_path)
                        }
                        continue;
                    }

                    #[cfg(unix)]
                    {
                        std::os::unix::fs::symlink(&source_path, &target_path)?;
                        if self.config.verbose {
                            info!("Created symlink: {:?} -> {:?}", source_path, target_path);
                        }
                    }

                    #[cfg(windows)]
                    {
                        // windows symlink creation (might require admin privs)
                        std::os::windows::fs::symlink_file(&source_path, &target_path)?;
                        if self.config.verbose {
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
            let relative_path = source_path.strip_prefix(source).unwrap();
            let target_path = target.join(relative_path);

            if file_type.is_dir() {
                if target_path.is_symlink() || self.is_dir_empty(&target_path) {
                    if self.config.simulate {
                        println!("Would remove directory: {:?}", target_path);
                    } else {
                        fs::remove_dir(&target_path)?;
                        if self.config.verbose {
                            info!("removed directory: {:?}", target_path);
                        }
                    }
                } else {
                    self.traverse_and_unlink(&source_path, &target_path)?;
                }
            } else if file_type.is_file() {
                // remove symlinks
                if target_path.is_symlink() {
                    if self.config.simulate {
                        println!("would remove symlink: {:?}", target_path);
                    } else {
                        fs::remove_file(&target_path)?;
                        if self.config.verbose {
                            info!("Removed symlink: {:?}", target_path);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// check if a directory is empty
    fn is_dir_empty(&self, path: &Path) -> bool {
        path.read_dir()
            .map_or(false, |mut entries| entries.next().is_none())
    }
}
