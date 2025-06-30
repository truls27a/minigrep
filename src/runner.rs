use std::{error::Error, fs};

use crate::config::Config;
use crate::model::Content;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.file_path)?;

    let content = Content::from_str(&text);
    let searched_content= content.search(&config.query, config.ignore_case, config.only_match_words, config.inverted_match);
    let highlighted_content = searched_content.highlight(&config.query);

    highlighted_content.display(config.show_line_numbers);

    Ok(())
}
