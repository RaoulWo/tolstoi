use std::{env, process};

use tolstoi::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing error: {err}");
        process::exit(1);
    });

    if let Err(err) = tolstoi::run(&config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
