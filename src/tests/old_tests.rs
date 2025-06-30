#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";

        assert_eq!(
            vec![Line::new(2, "safe, fast, productive.".to_string())],
            search(query, contents, false, false, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";

        assert_eq!(
            vec![
                Line::new(1, "Rust:".to_string()),
                Line::new(4, "Trust me.".to_string())
            ],
            search(query, contents, true, false, false)
        );
    }

    #[test]
    fn case_sensitive_only_match_words() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
I trust dust
I love rust.
";

        assert_eq!(
            vec![Line::new(5, "I love rust.".to_string())],
            search(query, contents, false, true, false)
        );
    }

    #[test]
    fn case_insensitive_only_match_words() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![
                Line::new(1, "Rust:".to_string()),
                Line::new(5, "I trust rust.".to_string())
            ],
            search(query, contents, true, true, false)
        );
    }

    #[test]
    fn case_sensitive_inverted_match() {
        let query = "s";
        let contents = "\
RuSt:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![
                Line::new(1, "RuSt:".to_string()),
                Line::new(3, "Pick three.".to_string())
            ],
            search(query, contents, false, false, true)
        );
    }

    #[test]
    fn case_insensitive_inverted_match() {
        let query = "R";
        let contents = "\
RuSt:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![] as Vec<Line>,
            search(query, contents, true, false, true)
        );
    }

    #[test]
    fn case_sensitive_only_match_words_inverted_match() {
        let query = "rust";
        let contents = "\
RuSt:
safe, fast, productive.
Pick three.
I trust dust
I trust rust.
";

        assert_eq!(
            vec![
                Line::new(1, "RuSt:".to_string()),
                Line::new(2, "safe, fast, productive.".to_string()),
                Line::new(3, "Pick three.".to_string()),
                Line::new(4, "I trust dust".to_string()),
            ],
            search(query, contents, false, true, true)
        );
    }
}
