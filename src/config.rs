use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_numbers: bool,
    pub only_match_words: bool,
    pub inverted_match: bool,
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