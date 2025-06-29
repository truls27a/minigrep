use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;

struct GrepConfig {
    query: String,
    file_path: String,
}

impl GrepConfig {
    fn new(env_args: &Vec<String>) -> Result<GrepConfig, &str> {
        if env_args.len() < 2 {
            return Err("Query and file path args missing!");
        } else if env_args.len() < 3 {
            return Err("File path arg missing!");
        }

        return Ok(GrepConfig {
            query: env_args[1].to_string(),
            file_path: env_args[2].to_string(),
        });
    }
}

fn parse_lines(file_path: &str) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file: File = File::open(file_path)?;
    let buff_reader: BufReader<File> = BufReader::new(file);
    let lines: std::io::Lines<BufReader<File>> = buff_reader.lines();
    return Ok(lines);
}

fn query_lines(lines: Lines<BufReader<File>>, query: &str) -> Vec<String> {
    let mut matching_lines: Vec<String> = Vec::new();

    for line_result in lines {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => {
                println!("Failed to read line!");
                String::new()
            }
        };

        if line.contains(query) {
            matching_lines.push(line)
        }
    };

    return matching_lines
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let grep_config: GrepConfig = match GrepConfig::new(&env_args) {
        Ok(grep_config) => grep_config,
        Err(message) => panic!("{message}"),
    };

    let lines = match parse_lines(&grep_config.file_path) {
        Ok(lines) => lines,
        Err(_) => panic!("Failed to read lines"),
    };

    let matching_lines: Vec<String> = query_lines(lines, &grep_config.query);

    println!("{:?}", matching_lines);

}
