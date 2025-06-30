pub struct Content {
    pub lines: Vec<Line>,
}

impl Content {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn from_str(text: &str) -> Self {
        let mut lines: Vec<Line> = Vec::new();

        for (index, text_line) in text.lines().enumerate() {
            let line = Line::new(
                index + 1, // Line index start at 1, not 0
                text_line.to_string(),
            );
            lines.push(line)
        }

        Self { lines }
    }

    pub fn search(&self, query: &str, ignore_case: bool, only_match_words: bool, inverted_match: bool) -> Self {
        let mut searched_content = Self::new();

        // Make query case aware
        let case_aware_query = if ignore_case {
            &query.to_lowercase()
        } else {
            query
        };


        for line in &self.lines {
            let maybe_searched_line = line.search(case_aware_query, ignore_case, only_match_words, inverted_match);

            match maybe_searched_line {
                Some(searched_line) => searched_content.lines.push(searched_line),
                None => {},
            }
            
        }

        searched_content
    }

    pub fn highlight(&self, query: &str) -> Self {
        let mut highlighted_content = Self::new();
        for line in &self.lines {
            let highlighted_text = line.highlight(query);
            highlighted_content.lines.push(
                Line::new(line.index, highlighted_text)
            );
        }

        highlighted_content
    }

    pub fn display(&self, show_line_numbers: bool) {
        for line in &self.lines {
            line.display(show_line_numbers);
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub index: usize,
    pub text: String,
}

impl Line {
    pub fn new(index: usize, text: String) -> Self {
        Self { index, text }
    }

    pub fn search(&self, query: &str, ignore_case: bool, only_match_words: bool, inverted_match: bool) -> Option<Self> {
        // Make line case aware
        let case_aware_line = if ignore_case {
            &self.text.to_lowercase()
        } else {
            &self.text
        };

        let mut line_matches = false;

        if only_match_words {
            let words: Vec<String> = case_aware_line
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
                .filter(|word| !word.is_empty())
                .map(|s| s.to_string())
                .collect();
            for word in words {
                if query == word {
                    line_matches = true;
                    break;
                }
            }
        } else {
            if case_aware_line.contains(query) {
                line_matches = true;
            };
        };

        let line: Option<Line> = if inverted_match {
            if !line_matches {
                Some(self.clone())
            } else {
                None
            }
        } else {
            if line_matches {
                Some(self.clone())
            } else {
                None
            }
        };

        line
    }

    pub fn highlight(&self, query: &str) -> String {
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

    pub fn display(&self, show_line_numbers: bool) {
        let text = &self.text;
            if show_line_numbers {
                let index = self.index;
                println!("{index}: {text}");
            } else {
                println!("{text}")
            }
    }
}
