use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::env;
use std::path::PathBuf;

/// Represents the configuration parsed from command-line arguments
#[derive(Debug, Clone)]
pub struct StowConfig {
    pub target_dir: PathBuf,
    pub stow_dir: PathBuf,
    pub package_name: String,
    pub simulate: bool,
    /// Verbosity count from `-v`: 0 = quiet, 1 = info, 2+ = debug
    pub verbose: u8,
    /// WIP: parsed but not yet implemented (see README)
    #[allow(dead_code)]
    pub no_folding: bool,
    /// WIP: parsed but not yet implemented (see README)
    #[allow(dead_code)]
    pub adopt: bool,
    pub restow: bool,
    pub unstow: bool,
}

/// Build the clap command for crab-stow.
/// Exposed separately from `parse_args` so tests can parse argument
/// vectors without spawning the process.
pub fn build_command() -> Command {
    Command::new("crab-stow")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Nicolas Moon <nmoon@nickmoon.rocks>")
        .about("A Rust implementation of GNU Stow")
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .help("Set the target directory")
                .default_value("."),
        )
        .arg(
            Arg::new("stow")
                .short('d')
                .long("dir")
                .help("Set the stow directory")
                .default_value("."),
        )
        .arg(
            Arg::new("simulate")
                .short('n')
                .long("no-act")
                .help("Do not actually make any changes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Increase verbosity (-v for info, -v -v for debug)")
                .action(ArgAction::Count),
        )
        .arg(
            Arg::new("restow")
                .short('R')
                .long("restow")
                .help("Restow (unlink and then stow)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("adopt")
                .long("adopt")
                .help("Adopt existing files into stow (WIP)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-folding")
                .long("no-folding")
                .help("Disable directory folding (WIP)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("package")
                .index(1)
                .required(true)
                .help("Package to stow/unstow"),
        )
        .arg(
            Arg::new("unstow")
                .short('D')
                .long("delete")
                .help("Unstow the package")
                .action(ArgAction::SetTrue),
        )
}

/// Parse command-line arguments and return a StowConfig
pub fn parse_args() -> Result<StowConfig> {
    let matches = build_command().get_matches();
    let current_dir = env::current_dir()?;

    // `get_one` can only fail if clap misbehaves: the package argument is
    // required and target/stow have defaults, so unwrapping is safe here.
    Ok(StowConfig {
        target_dir: current_dir.join(matches.get_one::<String>("target").unwrap()),
        stow_dir: current_dir.join(matches.get_one::<String>("stow").unwrap()),
        package_name: matches.get_one::<String>("package").unwrap().to_string(),
        simulate: matches.get_flag("simulate"),
        verbose: matches.get_count("verbose"),
        no_folding: matches.get_flag("no-folding"),
        adopt: matches.get_flag("adopt"),
        restow: matches.get_flag("restow"),
        unstow: matches.get_flag("unstow"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_args() {
        let matches = build_command()
            .try_get_matches_from(vec!["crab-stow", "dotfiles"])
            .unwrap();

        assert_eq!(matches.get_one::<String>("package").unwrap(), "dotfiles");
        assert!(!matches.get_flag("simulate"));
        assert!(!matches.get_flag("unstow"));
        assert!(!matches.get_flag("restow"));
        assert_eq!(matches.get_count("verbose"), 0);
    }

    #[test]
    fn parses_flags_and_counts_verbosity() {
        let matches = build_command()
            .try_get_matches_from(vec![
                "crab-stow",
                "-n",
                "-v",
                "-v",
                "-D",
                "-R",
                "-t",
                "/tmp",
                "-d",
                "/packages",
                "dotfiles",
            ])
            .unwrap();

        assert!(matches.get_flag("simulate"));
        assert!(matches.get_flag("unstow"));
        assert!(matches.get_flag("restow"));
        assert_eq!(matches.get_count("verbose"), 2);
        assert_eq!(matches.get_one::<String>("target").unwrap(), "/tmp");
        assert_eq!(matches.get_one::<String>("stow").unwrap(), "/packages");
    }

    #[test]
    fn package_argument_is_required() {
        let err = build_command()
            .try_get_matches_from(vec!["crab-stow"])
            .unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }
}
