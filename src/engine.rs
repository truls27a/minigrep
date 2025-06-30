use crate::format::Line;

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    ignore_case: bool,
    only_match_words: bool,
    inverted_match: bool,
) -> Vec<Line> {
    // Make query case aware
    let case_aware_query = if ignore_case {
        &query.to_lowercase()
    } else {
        query
    };

    let mut results: Vec<Line> = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        // Make line case aware
        let case_aware_line = if ignore_case {
            &line.to_lowercase()
        } else {
            line
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
                if case_aware_query == word {
                    line_matches = true;
                    break;
                }
            }
        } else {
            if case_aware_line.contains(case_aware_query) {
                line_matches = true;
            };
        };

        if inverted_match {
            if !line_matches {
                let line_index = index + 1; // We add one since index starts at 0 while line index should start at 1
                let line_content = line.to_string();
                let matching_line = Line::new(line_index, line_content);
                results.push(matching_line);
            }
        } else {
            if line_matches {
                let line_index = index + 1; // We add one since index starts at 0 while line index should start at 1
                let line_content = line.to_string();
                let matching_line = Line::new(line_index, line_content);
                results.push(matching_line);
            }
        }
    }

    results
}