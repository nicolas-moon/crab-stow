mod cli;
mod stow;
mod config {
    pub use crate::cli::StowConfig;
}

use anyhow::Result;
use env_logger;

fn main() -> Result<()> {
    //init the logger
    env_logger::init();

    // init config
    let config = cli::parse_args();

    // create crabStow instance and execute
    let crab_stow = stow::CrabStow::new(config);
    crab_stow.execute()?;

    Ok(())
}
