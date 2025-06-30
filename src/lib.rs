use colored::*;
use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    show_line_numbers: bool,
    only_match_words: bool,
    inverted_match: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[&args.len() - 2].clone(); // Second last arg

        let file_path = args[&args.len() - 1].clone(); // Last arg

        let ignore_case = if args.contains(&"-i".to_string()) {
            true
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        let show_line_numbers = if args.contains(&"-n".to_string()) {
            true
        } else {
            env::var("SHOW_LINE_NUMBERS").is_ok()
        };

        let only_match_words = if args.contains(&"-w".to_string()) {
            true
        } else {
            env::var("ONLY_MATCH_WORDS").is_ok()
        };

        let inverted_match = if args.contains(&"-v".to_string()) {
            true
        } else {
            env::var("INVERTED_MATCH").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
            only_match_words,
            inverted_match,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    index: usize,
    content: String,
}

impl Line {
    fn new(index: usize, content: String) -> Line {
        Line { index, content }
    }
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    ignore_case: bool,
    only_match_words: bool,
    inverted_match: bool,
) -> Vec<Line> {
    // Make query case aware
    let case_aware_query = if ignore_case {
        &query.to_lowercase()
    } else {
        query
    };

    let mut results: Vec<Line> = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        // Make line case aware
        let case_aware_line = if ignore_case {
            &line.to_lowercase()
        } else {
            line
        };

        let mut line_matches = false;

        if only_match_words {
            let words: Vec<String> = case_aware_line
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
                .filter(|word| !word.is_empty())
                .map(|s| s.to_string())
                .collect();
            for word in words {
                if case_aware_query == word {
                    line_matches = true;
                    break;
                };
            }
        } else {
            if case_aware_line.contains(case_aware_query) {
                line_matches = true;
            };
        };

        if inverted_match {
            if !line_matches {
                let line_index = index + 1; // We add one since index starts at 0 while line index should start at 1
                let line_content = line.to_string();
                let matching_line = Line::new(line_index, line_content);
                results.push(matching_line);
            };
        } else {
            if line_matches {
                let line_index = index + 1; // We add one since index starts at 0 while line index should start at 1
                let line_content = line.to_string();
                let matching_line = Line::new(line_index, line_content);
                results.push(matching_line);
            };
        };
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let lines = search(
        &config.query,
        &contents,
        config.ignore_case,
        config.only_match_words,
        config.inverted_match,
    );

    for line in lines {
        let colored_content = line
            .content
            .split_whitespace()
            .map(|word| {
                if word == config.query {
                    word.red().to_string()
                } else {
                    word.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        if config.show_line_numbers {
            let index = line.index;
            println!("{index}: {colored_content}");
        } else {
            println!("{colored_content}")
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";

        assert_eq!(
            vec![Line::new(2, "safe, fast, productive.".to_string())],
            search(query, contents, false, false, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";

        assert_eq!(
            vec![
                Line::new(1, "Rust:".to_string()),
                Line::new(4, "Trust me.".to_string())
            ],
            search(query, contents, true, false, false)
        );
    }

    #[test]
    fn case_sensitive_only_match_words() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
I trust dust
I love rust.
";

        assert_eq!(
            vec![Line::new(5, "I love rust.".to_string())],
            search(query, contents, false, true, false)
        );
    }

    #[test]
    fn case_insensitive_only_match_words() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![
                Line::new(1, "Rust:".to_string()),
                Line::new(5, "I trust rust.".to_string())
            ],
            search(query, contents, true, true, false)
        );
    }

    #[test]
    fn case_sensitive_inverted_match() {
        let query = "s";
        let contents = "\
RuSt:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![
                Line::new(1, "RuSt:".to_string()),
                Line::new(3, "Pick three.".to_string())
            ],
            search(query, contents, false, false, true)
        );
    }

    #[test]
    fn case_insensitive_inverted_match() {
        let query = "R";
        let contents = "\
RuSt:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![] as Vec<Line>,
            search(query, contents, true, false, true)
        );
    }

    #[test]
    fn case_sensitive_only_match_words_inverted_match() {
        let query = "rust";
        let contents = "\
RuSt:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![
                Line::new(1, "RuSt:".to_string()),
                Line::new(2, "safe, fast, productive.".to_string()),
                Line::new(3, "Pick three.".to_string()),
                Line::new(4, "I trust dust".to_string()),
            ],
            search(query, contents, false, true, true)
        );
    }
}
