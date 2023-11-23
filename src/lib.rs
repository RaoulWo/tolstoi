mod config;

pub use config::Config;

use std::{error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read the files.
    let book = fs::read_to_string(&config.book_fp)?;
    let peace = fs::read_to_string(&config.peace_terms_fp)?;
    let war = fs::read_to_string(&config.war_terms_fp)?;

    // Chapterize the book.
    let chapters: Vec<String> = chapterize(&book);
    // Tokenize the chapters.
    let chapters: Vec<Vec<String>> = tokenize(&chapters);

    // Convert peace & war terms to trimmed lines.
    let peace = to_trimmed_lines(&peace);
    let war = to_trimmed_lines(&war);

    // Calculate term density for each chapter and print result.
    for (i, chapter) in chapters.iter().enumerate() {
        let peace_density = calculate_term_density(&chapter, &peace);
        let war_density = calculate_term_density(&chapter, &war);

        print!("CHAPTER {i}: ");

        if peace_density > war_density {
            println!("Peace related!");
        } else if peace_density < war_density {
            println!("War related!");
        } else {
            println!("Equal density!");
        }
    }

    Ok(())
}

fn to_trimmed_lines<'a>(string: &'a str) -> Vec<&'a str> {
    string.lines().map(|line| line.trim()).collect()
}

fn chapterize(book: &str) -> Vec<String> {
    let mut chapters: Vec<Vec<&str>> = vec![];

    for line in book.lines() {
        if is_chapter_start(line) {
            chapters.push(vec![]);
        } else if is_book_end(line) {
            break;
        };

        if !chapters.is_empty() {
            chapters.last_mut().unwrap().push(line);
        }
    }

    let chapters: Vec<String> = chapters.iter().map(|chapter| chapter.join(" ")).collect();

    chapters
}

fn is_book_end(line: &str) -> bool {
    line.contains("END OF THE PROJECT GUTENBERG EBOOK, WAR AND PEACE")
}

fn is_chapter_start(line: &str) -> bool {
    line.contains("CHAPTER ")
}

fn tokenize(chapters: &Vec<String>) -> Vec<Vec<String>> {
    let mut tokenized_chapters = vec![];

    for chapter in chapters {
        tokenized_chapters.push(
            chapter
                .split_whitespace()
                .map(|token| token.trim_matches(|c| !char::is_alphabetic(c)))
                .filter(|token| !token.is_empty())
                .map(|token| String::from(token))
                .collect(),
        );
    }

    tokenized_chapters
}

fn calculate_term_density(tokenized_chapter: &Vec<String>, terms: &Vec<&str>) -> f32 {
    calculate_term_count(tokenized_chapter, terms) as f32
        / (tokenized_chapter.len() as f32
            * calculate_avg_nearest_term_dist(tokenized_chapter, terms))
}

fn calculate_term_count(tokenized_chapter: &Vec<String>, terms: &Vec<&str>) -> usize {
    let mut term_count: usize = 0;

    for token in tokenized_chapter {
        if terms.contains(&token.as_str()) {
            term_count += 1;
        }
    }

    term_count
}

fn calculate_avg_nearest_term_dist(tokenized_chapter: &Vec<String>, terms: &Vec<&str>) -> f32 {
    let filtered_tokens: Vec<(usize, &String)> = tokenized_chapter
        .iter()
        .enumerate()
        .filter(|(i, token)| terms.contains(&token.as_str()))
        .collect();

    let mut sum_nearest_term_dist: usize = 0;

    for (i, token) in filtered_tokens.iter().enumerate() {
        if i > 0 {
            sum_nearest_term_dist += token.0 - filtered_tokens[i - 1].0;
        }
    }

    let sum_nearest_term_dist = if sum_nearest_term_dist == 0 {
        1
    } else {
        sum_nearest_term_dist
    };

    if filtered_tokens.len() == 0 {
        1_000_000.0 // An arbitrarily large number to calculate a small term density.
    } else {
        sum_nearest_term_dist as f32 / filtered_tokens.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_trim_string() {
        let string = String::from(" foo  ");
        let result = to_trimmed_lines(&string);
        assert_eq!(result, vec!["foo"]);
    }

    #[test]
    fn should_split_string_into_lines() {
        let string = String::from("foo\nbar");
        let result = to_trimmed_lines(&string);
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn should_split_string_into_trimmed_lines() {
        let string = String::from("  foo \n  bar ");
        let result = to_trimmed_lines(&string);
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn should_chapterize_book_correctly() {
        let book = "\
Before chapter.
CHAPTER 1
Content 1
CHAPTER 2
Content 2
**** END OF THE PROJECT GUTENBERG EBOOK, WAR AND PEACE ****
After book end.";
        let chapters = chapterize(book);
        let expected = vec!["CHAPTER 1 Content 1", "CHAPTER 2 Content 2"];
        assert_eq!(chapters, expected);
    }

    #[test]
    fn line_should_end_book() {
        let line = "**** END OF THE PROJECT GUTENBERG EBOOK, WAR AND PEACE ****";
        let result = is_book_end(line);
        assert_eq!(result, true);
    }

    #[test]
    fn line_should_not_end_book() {
        let line = "**** END OF THE PROJECT GUTENBERG EBOOK WAR AND PEACE ****";
        let result = is_book_end(line);
        assert_eq!(result, false);
    }

    #[test]
    fn line_should_start_chapter() {
        let line = "CHAPTER 1";
        let result = is_chapter_start(line);
        assert_eq!(result, true);
    }

    #[test]
    fn line_should_not_start_chapter() {
        let line = "chapter 1";
        let result = is_chapter_start(line);
        assert_eq!(result, false);
    }

    #[test]
    fn should_tokenize_chapters_correctly() {
        let book = "\
Before chapter.
CHAPTER 1
Content 1
CHAPTER 2
Content 2
**** END OF THE PROJECT GUTENBERG EBOOK, WAR AND PEACE ****
After book end.";
        let chapters = chapterize(book);
        let tokenized_chapters = tokenize(&chapters);
        let expected = vec![vec!["CHAPTER", "Content"], vec!["CHAPTER", "Content"]];
        assert_eq!(tokenized_chapters, expected);
    }

    #[test]
    fn calculate_term_density_should_return_1() {
        let tokenized_chapter = vec![String::from("foo")];
        let terms = vec!["foo"];
        let term_density = calculate_term_density(&tokenized_chapter, &terms);
        assert!((term_density - 1.0).abs() <= 0.001);
    }

    #[test]
    fn calculate_term_density_should_return_0() {
        let tokenized_chapter = vec![String::from("bar")];
        let terms = vec!["foo"];
        let term_density = calculate_term_density(&tokenized_chapter, &terms);
        assert!((term_density - 0.0).abs() <= 0.001);
    }

    #[test]
    fn calculate_term_density_should_return_0p5() {
        let tokenized_chapter = vec![
            String::from("foo"),
            String::from("bar"),
            String::from("fo"),
            String::from("baar"),
        ];
        let terms = vec!["foo", "fo"];
        let term_density = calculate_term_density(&tokenized_chapter, &terms);
        println!("{term_density}");
        assert!((term_density - 0.5).abs() <= 0.001);
    }

    #[test]
    fn calculate_term_count_should_return_1() {
        let tokenized_chapter = vec![
            String::from("foo"),
            String::from("bar"),
            String::from("fo"),
            String::from("baar"),
        ];
        let terms = vec!["foo"];
        let term_count = calculate_term_count(&tokenized_chapter, &terms);
        assert_eq!(term_count, 1);
    }

    #[test]
    fn calculate_term_count_should_return_0() {
        let tokenized_chapter = vec![
            String::from("bar"),
            String::from("fo"),
            String::from("baar"),
        ];
        let terms = vec!["foo"];
        let term_count = calculate_term_count(&tokenized_chapter, &terms);
        assert_eq!(term_count, 0);
    }

    #[test]
    fn calculate_nearest_term_dist_should_return_1() {
        let tokenized_chapter = vec![String::from("bar")];
        let terms = vec!["bar"];
        let avg_nearest_term_dist = calculate_avg_nearest_term_dist(&tokenized_chapter, &terms);
        assert!((avg_nearest_term_dist - 1.0).abs() <= 0.001);
    }

    #[test]
    fn calculate_nearest_term_dist_should_return_1p5() {
        let tokenized_chapter = vec![
            String::from("bar"),
            String::from("foo"),
            String::from("foo"),
            String::from("bar"),
        ];
        let terms = vec!["bar"];
        let avg_nearest_term_dist = calculate_avg_nearest_term_dist(&tokenized_chapter, &terms);
        assert!((avg_nearest_term_dist - 1.5).abs() <= 0.001);
    }

    #[test]
    fn calculate_nearest_term_dist_should_return_1_000_000() {
        let tokenized_chapter = vec![];
        let terms = vec!["bar"];
        let avg_nearest_term_dist = calculate_avg_nearest_term_dist(&tokenized_chapter, &terms);
        assert!((avg_nearest_term_dist - 1_000_000.0).abs() <= 0.001);
    }
}
