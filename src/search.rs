use crate::model::{Content, Line};

impl Content {
    pub fn search(
        self,
        query: &str,
        ignore_case: bool,
        only_match_words: bool,
        inverted_match: bool,
    ) -> Self {
        let mut searched_content = Self::new();

        for line in self.lines {
            let maybe_searched_line = line.search(
                query,
                ignore_case,
                only_match_words,
                inverted_match,
            );

            match maybe_searched_line {
                Some(searched_line) => searched_content.lines.push(searched_line),
                None => {}
            }
        }

        searched_content
    }
}

impl Line {
    pub fn search(
        self,
        query: &str,
        ignore_case: bool,
        only_match_words: bool,
        inverted_match: bool,
    ) -> Option<Self> {
        // Make text case aware
        let case_aware_text = if ignore_case {
            &self.text.to_lowercase()
        } else {
            &self.text
        };

        let case_aware_query = if ignore_case {
            &query.to_lowercase()
        } else {
            query
        };

        let mut line_matches = false;

        if only_match_words {
            let words: Vec<String> = case_aware_text
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
            if case_aware_text.contains(case_aware_query) {
                line_matches = true;
            };
        };

        let line: Option<Line> = if inverted_match {
            if !line_matches { Some(self) } else { None }
        } else {
            if line_matches { Some(self) } else { None }
        };

        line
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_search_returns_line_if_containing_query() {
        let line = Line::new(1, String::from("This minigrep tool is amazing!"));

        let maybe_searched_line = line.clone().search("minigrep", false, false, false);

        assert_eq!(Some(line), maybe_searched_line)
    }

    #[test]
    fn line_search_does_not_return_line_if_not_containing_query() {
        let line = Line::new(1, String::from("This minigrep tool is amazing!"));

        let maybe_searched_line = line.search("hello", false, false, false);

        assert_eq!(None, maybe_searched_line)
    }
    
    #[test]
    fn line_search_ignore_case_when_enabled() {
        let line = Line::new(1, String::from("Me too"));

        let maybe_searched_line = line.clone().search("TO", true, false, false);

        assert_eq!(Some(line), maybe_searched_line)
    }

    #[test]
    fn line_search_does_not_ignore_case_when_disabled() {
        let line = Line::new(1, String::from("Me too"));

        let maybe_searched_line = line.search("TO", false, false, false);

        assert_eq!(None, maybe_searched_line)
    }

    #[test]
    fn line_search_only_matches_words_when_enabled() {
        let line = Line::new(1, String::from("Me too"));

        let maybe_searched_line = line.search("to", false, true, false);

        assert_eq!(None, maybe_searched_line)
    }

    #[test]
    fn line_search_does_not_only_match_words_when_disabled() {
        let line = Line::new(1, String::from("Me too"));

        let maybe_searched_line = line.clone().search("to", false, false, false);

        assert_eq!(Some(line), maybe_searched_line)
    }

    #[test]
    fn line_search_inverts_match_when_enabled() {
        let line = Line::new(1, String::from("Me too"));

        let maybe_searched_line = line.search("to", false, false, true);

        assert_eq!(None, maybe_searched_line)
    }

    #[test]
    fn line_search_does_not_invert_match_when_disabled() {
        let line = Line::new(1, String::from("Me too"));

        let maybe_searched_line = line.clone().search("to", false, false, false);

        assert_eq!(Some(line), maybe_searched_line)
    }


    #[test]
    fn line_search_ignore_case_only_match_words_inverted_match_work_togheter() {
        let line = Line::new(1, String::from("Then there's a pair of us - don't tell!"));

        let maybe_searched_line = line.clone().search("AI", true, true, true);

        assert_eq!(Some(line), maybe_searched_line)
    }
}
