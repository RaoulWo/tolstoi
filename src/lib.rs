mod config;

pub use config::Config;

use std::{error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read the files.
    let book = fs::read_to_string(&config.book_fp)?;
    let peace = fs::read_to_string(&config.peace_terms_fp)?;
    let war = fs::read_to_string(&config.war_terms_fp)?;

    Ok(())
}
