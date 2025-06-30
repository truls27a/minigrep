use std::{error::Error, fs};

use crate::config::Config;
use crate::engine;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let lines = engine::search(
        &config.query,
        &contents,
        config.ignore_case,
        config.only_match_words,
        config.inverted_match,
    );

    for line in lines {
        let colored_content = match config.inverted_match {
            true => line.content,
            false => line.highlight(&config.query),
        };

        if config.show_line_numbers {
            let index = line.index;
            println!("{index}: {colored_content}");
        } else {
            println!("{colored_content}")
        }
    }

    Ok(())
}
