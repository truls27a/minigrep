use crate::model::{Content, Line};

impl Content {
    pub fn highlight(&self, query: &str) -> Self {
        let mut highlighted_content = Self::new();
        for line in &self.lines {
            let highlighted_text = line.highlight(query);
            highlighted_content
                .lines
                .push(Line::new(line.index, highlighted_text));
        }

        highlighted_content
    }
}

impl Line {
    pub fn highlight(&self, query: &str) -> String {
        // TODO: This should take in the case sensative config in order to know if it should higlight case sensitaviably or not
        let mut colored_content = String::new();

        let content_chars: Vec<char> = self.text.chars().collect();
        let mut content_index = 0;

        let query_chars: Vec<char> = query.chars().collect();
        let mut matching_query_index = 0;

        while content_index < content_chars.len() {
            colored_content.push(content_chars[content_index]);

            if content_chars[content_index] == query_chars[matching_query_index] {
                matching_query_index += 1;
            } else {
                matching_query_index = 0;
            }

            if matching_query_index == query_chars.len() {
                colored_content.insert_str(content_index - matching_query_index + 1, "\x1b[31m");
                colored_content.push_str("\x1b[0m");
                matching_query_index = 0;
            }

            content_index += 1;
        }

        colored_content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_highlight_wraps_matching_query_in_red() {
        let line = Line::new(1, String::from("I am a line."));

        let highlighted_text = line.highlight("line");

        assert_eq!("I am a \x1b[31mline\x1b[0m.", highlighted_text)
    }

    #[test]
    fn no_line_highlight_when_query_not_found() {
        let line = Line::new(1, String::from("I am a line."));

        let highlighted_text = line.highlight("monkey");

        assert_eq!(line.text, highlighted_text)
    }

    #[test]
    fn line_highlight_case_sensative_works() {
        assert_eq!(true, false) // TODO: Impliment case sensative in line hilighting
    }

    #[test]
    fn content_highlight_wraps_matching_query_in_red_in_lines() {
        let content = Content::from_lines(vec![
            Line::new(1, String::from("I am a line.")),
            Line::new(2, String::from("I am also line.")),
            Line::new(3, String::from("Me too!")),
            Line::new(3, String::from("Line")),
        ]);

        let highlighted_content = content.highlight("line");

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
