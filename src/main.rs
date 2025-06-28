use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("Search and file path args missing!");
    } else if args.len() < 3 {
        panic!("File path arg missing!");
    }
    let search = &args[1];
    let file_path = &args[2];


    let file = File::open(file_path)?;
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

        let words = line.split_whitespace();
        for word in words {
            if word == search {
                matching_lines.push(line);
                break
            }
        }
    }

    println!("{:?}", matching_lines);

    Ok(())
}
