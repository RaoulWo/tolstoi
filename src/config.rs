#[derive(Debug, PartialEq)]
pub struct Config {
    pub book_fp: String,
    pub peace_terms_fp: String,
    pub war_terms_fp: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let book_fp = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a book file path"),
        };

        let peace_terms_fp = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a peace terms file path"),
        };

        let war_terms_fp = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a war terms file path"),
        };

        Ok(Config {
            book_fp,
            peace_terms_fp,
            war_terms_fp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_throw_err_missing_book_fp() {
        let args = vec!["tolstoi".to_string()];
        let err = Config::build(args.into_iter());
        let expected = Err("Didn't get a book file path");
        assert_eq!(err, expected);
    }

    #[test]
    fn should_throw_err_missing_peace_terms_fp() {
        let args = vec!["tolstoi".to_string(), "book".to_string()];
        let err = Config::build(args.into_iter());
        let expected = Err("Didn't get a peace terms file path");
        assert_eq!(err, expected);
    }

    #[test]
    fn should_throw_err_missing_war_terms_fp() {
        let args = vec![
            "tolstoi".to_string(),
            "book".to_string(),
            "peace".to_string(),
        ];
        let err = Config::build(args.into_iter());
        let expected = Err("Didn't get a war terms file path");
        assert_eq!(err, expected);
    }

    #[test]
    fn should_return_config() {
        let args = vec![
            "tolstoi".to_string(),
            "book".to_string(),
            "peace".to_string(),
            "war".to_string(),
        ];
        let config = Config::build(args.into_iter());
        let expected = Config {
            book_fp: "book".to_string(),
            peace_terms_fp: "peace".to_string(),
            war_terms_fp: "war".to_string(),
        };
        assert_eq!(config, Ok(expected));
    }
}
