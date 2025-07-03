use crate::model::{Content, Line};

impl Content {
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
}

impl Line {
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
                .map(|s| String::from(s))
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
}