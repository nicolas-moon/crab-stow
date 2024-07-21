use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod operate;

#[derive(Parser)]
#[command(name = "crab-stow")]
#[command(about = "Stow CLI reimagined in Rust", version = "0.0.1", author = "Nicolas Moon")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Stow {
        #[arg(help = "The package directory to stow")]
        package: PathBuf,
        #[arg(help = "The Target directory")]
        target: PathBuf
    }
}


fn main() {
    let cli = Cli::parse()

    match &cli.command {
        Commands::Stow { package, target }.expect("Failed to stow");
    }
}
