use crate::model::{Content, Line};

impl Content {
    pub fn highlight(&self, query: &str, ignore_case: bool) -> Self {
        let mut highlighted_content = Self::new();
        for line in &self.lines {
            let highlighted_text = line.highlight(query, ignore_case);
            highlighted_content
                .lines
                .push(Line::new(line.index, highlighted_text));
        }

        highlighted_content
    }
}

impl Line {
    pub fn highlight(&self, query: &str, ignore_case: bool) -> String {
        let mut highlighted_text = String::new();

        // If ignore case is enabled, we want to compare in lowercase but still keep the normal case chars for display. Hence the duplicate vecs
        let text_chars: Vec<char> = self.text.chars().collect();
        let case_aware_text_chars: Vec<char> = if ignore_case {
            self.text.to_lowercase().chars().collect()
        } else {
            self.text.chars().collect()
        };
        let mut text_index = 0;

        // Same here, we want to compare in lowercase but display normal case
        let query_chars: Vec<char> = query.chars().collect();
        let case_aware_query_chars: Vec<char> = if ignore_case {
            query.to_lowercase().chars().collect()
        } else {
            query.chars().collect()
        };

        // matching_query_index represent the number of previous consecutive matches in chars for between the content and query
        // So if matching_query_index is for example 3, that means that the past three chars in text_chars have been the same as the first 3 chars in query_chars
        let mut matching_query_index = 0;

        // Map through each char in the line text
        // For each char, check if the matching_query_index (see explination above) is the the same as the length of the query
        // If so, push the query with color to the highlighted_text and reset matching_query_index
        // Else, push the char without color
        // Then, check if the current char is the same as the query char for the current matching_query_index
        // If so, incriment it. Else reset it to 0
        // Finally, incriment text_index to move on to the next char
        while text_index < text_chars.len() {
            if matching_query_index == query_chars.len() {
                highlighted_text.insert_str(text_index - matching_query_index, "\x1b[31m");
                highlighted_text.push_str("\x1b[0m");
                matching_query_index = 0;
            }

            highlighted_text.push(text_chars[text_index]);

            if case_aware_text_chars[text_index] == case_aware_query_chars[matching_query_index] {
                matching_query_index += 1;
            } else {
                matching_query_index = 0;
            }

            text_index += 1;
        }

        highlighted_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_highlight_wraps_matching_query_in_red() {
        let line = Line::new(1, String::from("I am a line."));

        let highlighted_text = line.highlight("line", false);

        assert_eq!("I am a \x1b[31mline\x1b[0m.", highlighted_text)
    }

    #[test]
    fn no_line_highlight_when_query_not_found() {
        let line = Line::new(1, String::from("I am a line."));

        let highlighted_text = line.highlight("monkey", false);

        assert_eq!(line.text, highlighted_text)
    }

    #[test]
    fn line_highlight_case_sensative_works() {
        assert_eq!(true, false) // TODO: Impliment case sensative in line hilighting
    }

    #[test]
    fn text_highlight_wraps_matching_query_in_red_in_lines() {
        let content = Content::from_lines(vec![
            Line::new(1, String::from("I am a line.")),
            Line::new(2, String::from("I am also line.")),
            Line::new(3, String::from("Me too!")),
            Line::new(3, String::from("Line")),
        ]);

        let highlighted_content = content.highlight("line", false);

        assert_eq!(
            Content::from_lines(vec![
                Line::new(1, String::from("I am a \x1b[31mline\x1b[0m.")),
                Line::new(2, String::from("I am also \x1b[31mline\x1b[0m.")),
                Line::new(3, String::from("Me too!")),
                Line::new(3, String::from("Line")),
            ]),
            highlighted_content
        )
    }
}
