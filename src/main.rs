use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct GrepConfig {
    query: String,
    file_path: String,
    case_sensitive: bool,
}

impl GrepConfig {
    fn new(env_args: &[String]) -> Result<GrepConfig, String> {
        if env_args.len() < 2 {
            return Err("Query and file path args missing!".to_string());
        } else if env_args.len() < 3 {
            return Err("File path arg missing!".to_string());
        }

        let mut case_sensitive = false;
        if &env_args[3] == "true" {
            case_sensitive = true;
        }

        Ok(GrepConfig {
            query: env_args[1].to_string(),
            file_path: env_args[2].to_string(),
            case_sensitive: case_sensitive,
        })
    }
}

fn parse_lines(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file: File = File::open(file_path)?;
    let buff_reader: BufReader<File> = BufReader::new(file);
    let lines = buff_reader.lines();

    let mut formated_lines: Vec<String> = Vec::new();
    for line in lines {
        let formated_line = match line {
            Ok(line) => line,
            Err(_) => String::new()
        };
        formated_lines.push(formated_line)
    }
    
    Ok(formated_lines)
}

fn query_lines<'a>(lines: &'a [String], query: &str, case_sensitive: bool) -> Vec<&'a str> {
    let mut matching_lines: Vec<&str> = Vec::new();
    let query_lower_case = query.to_lowercase();

    for line in lines {
        if case_sensitive {
            if line.contains(&query) {
                matching_lines.push(line);
            }
        } else {
            if line.to_lowercase().contains(&query_lower_case) {
                matching_lines.push(line);
            }
        }
        
    };

    matching_lines
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_args: Vec<String> = env::args().collect();
    let config: GrepConfig = GrepConfig::new(&env_args)?;

    let lines = parse_lines(&config.file_path)?;

    let matches: Vec<&str> = query_lines(&lines, &config.query, config.case_sensitive);

    println!("{:?}", matches);
    Ok(())

}
