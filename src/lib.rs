mod config;

pub use config::Config;

use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
