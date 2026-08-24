use crab_stow::cli::StowConfig;
use crab_stow::stow::CrabStow;
use std::fs;
use std::path::PathBuf;

const PACKAGE: &str = "dotfiles";

fn make_config(target_dir: PathBuf, stow_dir: PathBuf, simulate: bool) -> StowConfig {
    StowConfig {
        target_dir,
        stow_dir,
        package_name: PACKAGE.to_string(),
        simulate,
        verbose: 0,
        no_folding: false,
        adopt: false,
        restow: false,
        unstow: false,
    }
}

fn make_config_with_flags(
    target_dir: PathBuf,
    stow_dir: PathBuf,
    simulate: bool,
    restow: bool,
    unstow: bool,
) -> StowConfig {
    StowConfig {
        target_dir,
        stow_dir,
        package_name: PACKAGE.to_string(),
        simulate,
        verbose: 0,
        no_folding: false,
        adopt: false,
        restow,
        unstow,
    }
}

/// Create a package with a top-level file and a nested file.
/// Returns the stow directory containing `packages/dotfiles`.
fn setup_package(root: &std::path::Path) -> PathBuf {
    let stow_dir = root.join("packages");
    let package = stow_dir.join(PACKAGE);
    fs::create_dir_all(package.join("vim")).unwrap();
    fs::write(package.join(".bashrc"), "export EDITOR=vim").unwrap();
    fs::write(package.join("vim").join("init.vim"), "set number").unwrap();
    stow_dir
}

#[test]
fn stow_creates_symlinks_for_files_and_dirs() {
    let tmp = tempfile::tempdir().unwrap();
    let stow_dir = setup_package(tmp.path());
    let target_dir = tmp.path().to_path_buf();
    let destination = target_dir.join(PACKAGE);

    CrabStow::new(make_config(target_dir.clone(), stow_dir, false))
        .stow()
        .unwrap();

    // top-level file is a symlink with the source content
    let bashrc = destination.join(".bashrc");
    assert!(bashrc.is_symlink());
    assert_eq!(fs::read_to_string(&bashrc).unwrap(), "export EDITOR=vim");

    // nested directory is a real dir containing a symlinked file
    let vim_dir = destination.join("vim");
    assert!(vim_dir.is_dir());
    assert!(!vim_dir.is_symlink());
    let init = vim_dir.join("init.vim");
    assert!(init.is_symlink());
    assert_eq!(fs::read_to_string(&init).unwrap(), "set number");
}

#[test]
fn stow_simulate_makes_no_changes() {
    let tmp = tempfile::tempdir().unwrap();
    let stow_dir = setup_package(tmp.path());
    let target_dir = tmp.path().to_path_buf();
    let destination = target_dir.join(PACKAGE);

    CrabStow::new(make_config(target_dir, stow_dir, true))
        .stow()
        .unwrap();

    // simulate mode must not touch the target at all
    assert!(!destination.exists());
}

#[test]
fn stow_unstow_roundtrip_restores_target() {
    let tmp = tempfile::tempdir().unwrap();
    let stow_dir = setup_package(tmp.path());
    let target_dir = tmp.path().to_path_buf();
    let destination = target_dir.join(PACKAGE);

    // stow
    CrabStow::new(make_config(target_dir.clone(), stow_dir.clone(), false))
        .stow()
        .unwrap();
    assert!(destination.join(".bashrc").is_symlink());

    // unstow removes every symlink and the directories left behind
    CrabStow::new(make_config_with_flags(
        target_dir.clone(),
        stow_dir.clone(),
        false,
        false,
        true,
    ))
    .execute()
    .unwrap();
    assert!(!destination.exists());

    // the stowed package itself is untouched
    assert!(stow_dir.join(PACKAGE).join(".bashrc").exists());

    // re-stow works after an unstow
    CrabStow::new(make_config(target_dir, stow_dir, false))
        .stow()
        .unwrap();
    assert!(destination.join(".bashrc").is_symlink());
}

#[test]
fn restow_unstows_then_stows() {
    let tmp = tempfile::tempdir().unwrap();
    let stow_dir = setup_package(tmp.path());
    let target_dir = tmp.path().to_path_buf();
    let destination = target_dir.join(PACKAGE);

    CrabStow::new(make_config(target_dir.clone(), stow_dir.clone(), false))
        .stow()
        .unwrap();
    assert!(destination.join(".bashrc").is_symlink());

    CrabStow::new(make_config_with_flags(
        target_dir.clone(),
        stow_dir,
        false,
        true,
        false,
    ))
    .execute()
    .unwrap();

    // after restow the symlinks are back in place
    assert!(destination.join(".bashrc").is_symlink());
    assert!(destination.join("vim").join("init.vim").is_symlink());
}

#[test]
fn stow_skips_existing_target_files() {
    let tmp = tempfile::tempdir().unwrap();
    let stow_dir = setup_package(tmp.path());
    let target_dir = tmp.path().to_path_buf();
    let destination = target_dir.join(PACKAGE);

    // pre-existing real file in the target that the package also defines
    fs::create_dir_all(&destination).unwrap();
    fs::write(destination.join(".bashrc"), "user's own file").unwrap();

    CrabStow::new(make_config(target_dir, stow_dir, false))
        .stow()
        .unwrap();

    // the existing file is kept, not replaced by a symlink
    let bashrc = destination.join(".bashrc");
    assert!(!bashrc.is_symlink());
    assert_eq!(fs::read_to_string(bashrc).unwrap(), "user's own file");

    // unrelated package files are still stowed
    assert!(destination.join("vim").join("init.vim").is_symlink());
}

#[test]
fn unstow_missing_package_is_an_error() {
    let tmp = tempfile::tempdir().unwrap();
    let stow_dir = tmp.path().to_path_buf();
    let target_dir = tmp.path().to_path_buf();

    let crab = CrabStow::new(make_config_with_flags(
        target_dir, stow_dir, false, false, true,
    ));
    let err = crab.execute().unwrap_err();
    assert!(err.to_string().contains("Package dotfiles not found"));
}
