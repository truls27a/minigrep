use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

struct GrepConfig {
    query: String,
    file_path: String,
}

impl GrepConfig {
    fn new(env_args: &Vec<String>) -> Result<GrepConfig, &str> {
        if env_args.len() < 2 {
            return Err("Query and file path args missing!")
        } else if env_args.len() < 3 {
            return Err("File path arg missing!")
        }

        return Ok(GrepConfig {
            query: env_args[1].to_string(),
            file_path: env_args[2].to_string()
        })
    }
}

fn main() -> std::io::Result<()> {
    let env_args: Vec<String> = env::args().collect();
    let grep_config = match GrepConfig::new(&env_args) {
        Ok(grep_config) => grep_config,
        Err(message) => panic!("{message}")
    };
    


    let file = File::open(grep_config.file_path)?;
    let buff_reader = BufReader::new(file);

    let mut matching_lines = Vec::new();

    for line_result in buff_reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => {
                println!("Failed to read line!");
                String::new()
            }
        };

        if line.contains(&grep_config.query) {
            matching_lines.push(line)
        }
    }

    println!("{:?}", matching_lines);

    Ok(())
}
