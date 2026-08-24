use anyhow::Result;
use crab_stow::cli;
use crab_stow::stow::CrabStow;

fn main() -> Result<()> {
    // init config
    let config = cli::parse_args()?;

    // init the logger (RUST_LOG still takes precedence over -v)
    init_logger(config.verbose);

    // create crabStow instance and execute
    let crab_stow = CrabStow::new(config);
    crab_stow.execute()?;

    Ok(())
}

/// Initialize logging based on the verbosity count from `-v`.
/// 0 => error (quiet), 1 => info, 2+ => debug.
fn init_logger(verbose: u8) {
    let default_filter = match verbose {
        0 => "error",
        1 => "info",
        _ => "debug",
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_filter))
        .init();
}
