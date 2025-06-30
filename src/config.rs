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
    pub fn new(
        query: String,
        file_path: String,
        ignore_case: bool,
        show_line_numbers: bool,
        only_match_words: bool,
        inverted_match: bool,
    ) -> Self {
        Self {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
            only_match_words,
            inverted_match,
        }
    }

    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[&args.len() - 2].clone(); // Second last arg

        let file_path: String = args[&args.len() - 1].clone(); // Last arg

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

        Ok(Self {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
            only_match_words,
            inverted_match,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_creation() {
        let query = String::from("Rust!");
        let file_path = String::from("src/main.rs");
        let ignore_case = false;
        let show_line_numbers = true;
        let only_match_words = true;
        let inverted_match = false;

        let config = Config::new(
            query.clone(),
            file_path.clone(),
            ignore_case,
            show_line_numbers,
            only_match_words,
            inverted_match,
        );

        assert_eq!(config.query, query);
        assert_eq!(config.file_path, file_path);
        assert_eq!(config.ignore_case, ignore_case);
        assert_eq!(config.show_line_numbers, show_line_numbers);
        assert_eq!(config.only_match_words, only_match_words);
        assert_eq!(config.inverted_match, inverted_match);
    }

    #[test]
    fn config_build() {
        let query = String::from("To be, or not to be");
        let file_path = String::from("poem.txt");
        let ignore_case = true;
        let show_line_numbers = true;
        let only_match_words = false;
        let inverted_match = true;

        let mut args: Vec<String> = vec!["src/main.rs".into()];

        if ignore_case {
            args.push(String::from("-i"));
        }
        if show_line_numbers {
            args.push(String::from("-n"));
        }
        if only_match_words {
            args.push(String::from("-w"));
        }
        if inverted_match {
            args.push(String::from("-v"));
        }
        
        args.push(query.clone()); // Needs to be second last
        args.push(file_path.clone()); // Needs to be last

        let config = Config::build(&args).unwrap();

        assert_eq!(config.query, query);
        assert_eq!(config.file_path, file_path);
        assert_eq!(config.ignore_case, ignore_case);
        assert_eq!(config.show_line_numbers, show_line_numbers);
        assert_eq!(config.only_match_words, only_match_words);
        assert_eq!(config.inverted_match, inverted_match);
    }

    #[test]
    fn config_failed_build() {
        let file_path = String::from("poem.txt");
        let ignore_case = true;
        let show_line_numbers = true;
        let only_match_words = false;
        let inverted_match = true;

        let mut args: Vec<String> = vec!["src/main.rs".into()];

        if ignore_case {
            args.push(String::from("-i"));
        }
        if show_line_numbers {
            args.push(String::from("-n"));
        }
        if only_match_words {
            args.push(String::from("-w"));
        }
        if inverted_match {
            args.push(String::from("-v"));
        }
        
        args.push(file_path.clone()); // Needs to be last

        let config = Config::build(&args); // This is missing the query

        // Should fail due to the query missing
        assert!(config.is_err());
    }
}
