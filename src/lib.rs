mod config;

pub use config::Config;

use std::{error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read the files.
    let book = fs::read_to_string(&config.book_fp)?;
    let peace = fs::read_to_string(&config.peace_terms_fp)?;
    let war = fs::read_to_string(&config.war_terms_fp)?;

    // Convert peace & war terms to trimmed lines.
    let peace = to_trimmed_lines(peace);
    let war = to_trimmed_lines(war);

    Ok(())
}

fn to_trimmed_lines(string: String) -> Vec<String> {
    string
        .lines()
        .map(|line| String::from(line.trim()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_trim_string() {
        let string = String::from(" foo  ");
        let result = to_trimmed_lines(string);
        assert_eq!(result, vec!["foo"]);
    }

    #[test]
    fn should_split_string_into_lines() {
        let string = String::from("foo\nbar");
        let result = to_trimmed_lines(string);
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn should_split_string_into_trimmed_lines() {
        let string = String::from("  foo \n  bar ");
        let result = to_trimmed_lines(string);
        assert_eq!(result, vec!["foo", "bar"]);
    }
}
