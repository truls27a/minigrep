use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct GrepConfig {
    query: String,
    file_path: String,
    case_insensitive: bool,
}

impl GrepConfig {
    fn new(query: String, file_path: String, case_insensitive: bool) -> Self {
        GrepConfig {
            query,
            file_path,
            case_insensitive,
        }
    }

    fn from_env() -> Result<Self, String> {
        let env_args = &env::args().collect::<Vec<_>>();
        if env_args.len() < 3 {
            return Err("Usage: minigrep <query> <file_path>".into());
        }

        let query = env_args[1].clone();
        let file_path = env_args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self::new(query, file_path, case_insensitive))
    }
}

fn parse_lines(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file: File = File::open(file_path)?;
    let buff_reader: BufReader<File> = BufReader::new(file);
    let lines = buff_reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;
    Ok(lines)
}

fn query_lines<'a>(lines: &'a [String], query: &str, case_insensitive: bool) -> Vec<&'a str> {
    let mut matching_lines: Vec<&str> = Vec::new();
    let query_lower_case = query.to_lowercase();

    for line in lines {
        if case_insensitive {
            if line.to_lowercase().contains(&query_lower_case) {
                matching_lines.push(line);
            }
        } else {
            if line.contains(&query) {
                matching_lines.push(line);
            }
        }
    }

    matching_lines
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: GrepConfig = GrepConfig::from_env()?;

    let lines: Vec<String> = parse_lines(&config.file_path)?;

    let matches: Vec<&str> = query_lines(&lines, &config.query, config.case_insensitive);

    println!("{:?}", matches);
    Ok(())
}
