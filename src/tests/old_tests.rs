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
            vec![Line::new(2, String::from("safe, fast, productive."))],
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
                Line::new(1, String::from("Rust:")),
                Line::new(4, String::from("Trust me."))
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
            vec![Line::new(5, String::from("I love rust."))],
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
                Line::new(1, String::from("Rust:")),
                Line::new(5, String::from("I trust rust."))
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
                Line::new(1, String::from("RuSt:")),
                Line::new(3, String::from("Pick three."))
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
                Line::new(1, String::from("RuSt:")),
                Line::new(2, String::from("safe, fast, productive.")),
                Line::new(3, String::from("Pick three.")),
                Line::new(4, String::from("I trust dust")),
            ],
            search(query, contents, false, true, true)
        );
    }
}
