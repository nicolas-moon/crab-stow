use clap::{Arg, ArgAction, Command};
use std::env;
use std::path::PathBuf;

///Represents the configuration parsed from command-line arguments
#[derive(Debug, Clone)]
pub struct StowConfig {
    pub target_dir: PathBuf,
    pub stow_dir: PathBuf,
    pub package_name: String,
    pub simulate: bool,
    pub verbose: bool,
    pub no_folding: bool,
    pub adopt: bool,
    pub restow: bool,
    pub unstow: bool,
}

/// Parse command-line arguments and return a StowConfig
pub fn parse_args() -> StowConfig {
    let matches = Command::new("crab_stow")
        .version("0.1.0")
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
                .help("set the stow directory")
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
                .help("Increase verbosity")
                .action(ArgAction::SetTrue),
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
                .help("Adopt existing files into stow")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-folding")
                .long("no-folding")
                .help("Disable directory folding")
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
        .get_matches();

    StowConfig {
        target_dir: env::current_dir()
            .unwrap()
            .join(matches.get_one::<String>("target").unwrap()),
        stow_dir: env::current_dir()
            .unwrap()
            .join(matches.get_one::<String>("stow").unwrap()),
        package_name: matches.get_one::<String>("package").unwrap().to_string(),
        simulate: matches.get_flag("simulate"),
        verbose: matches.get_flag("verbose"),
        no_folding: matches.get_flag("no-folding"),
        adopt: matches.get_flag("adopt"),
        restow: matches.get_flag("restow"),
        unstow: matches.get_flag("unstow"),
    }
}
