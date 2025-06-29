use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
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

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[derive(Debug)]
pub struct Line {
    content: String,
    index: usize,
}

impl Line {
    fn new(content: String, index: usize) -> Line {
        Line{ content, index}
    }
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<Line> {
    // Make query case aware
    let case_aware_query = if ignore_case {
        &query.to_lowercase()
    } else {
        query
    };

    let mut results: Vec<Line> = Vec::new();

    for (index,line) in contents.lines().enumerate() {
        // Make line case aware
        let case_aware_line = if ignore_case {
            &line.to_lowercase()
        } else {
            line
        };

        if case_aware_line.contains(case_aware_query) {
            results.push(
                Line::new(line.to_string(), index)
            )
        }
    }

    results
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let lines = search(&config.query, &contents, config.ignore_case);

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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
