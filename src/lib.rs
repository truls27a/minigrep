use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    only_match_whole_words: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();

        let file_path = args[2].clone();

        let ignore_case = if args.len() > 3 {
            if args[3] == "true" || args[3] == "1" {
                true
            } else {
                false
            }
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        let only_match_whole_words = if args.len() > 3 {
            if args[4] == "true" || args[4] == "1" {
                true
            } else {
                false
            }
        } else {
            env::var("ONLY_MATCH_WHOLE_WORDS").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
            only_match_whole_words,
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
    only_match_whole_words: bool,
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

        if only_match_whole_words {
            let words: Vec<String> = case_aware_line
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
                .filter(|word| !word.is_empty())
                .map(|s| s.to_string())
                .collect();
            for word in words {
                println!("{}", word);
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

        if line_matches {
            let line_index = index + 1; // We add one since index starts at 0 while line index should start at 1
            let line_content = line.to_string();
            let matching_line = Line::new(line_index, line_content);
            results.push(matching_line);
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
        config.only_match_whole_words,
    );

    for line in lines {
        println!("{:?}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: Update tests to work
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
            search(query, contents, false, false)
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
            search(query, contents, true, false)
        );
    }

    #[test]
    fn only_match_whole_words_case_sensative() {
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
            search(query, contents, false, true)
        );
    }

    #[test]
    fn only_match_whole_words_case_insensative() {
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
            search(query, contents, true, true)
        );
    }
}
