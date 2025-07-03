#[derive(Debug, PartialEq)]
pub struct Content {
    pub lines: Vec<Line>,
}

impl Content {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn from_lines(lines: Vec<Line> ) -> Self {
        Self { lines }
    }

    pub fn from_str(text: &str) -> Self {
        let mut lines: Vec<Line> = Vec::new();

        for (index, text_line) in text.lines().enumerate() {
            let line = Line::new(
                index + 1, // Line index start at 1, not 0
                String::from(text_line),
            );
            lines.push(line)
        }

        Self { lines }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_from_lines_preserves_index_and_text() {
        let first_index = 3;
        let first_text = String::from("Hello World!");
        let second_index = 3;
        let second_text = String::from("Hello World!");

        let lines = vec![
            Line::new(first_index, first_text.clone()),
            Line::new(second_index, second_text.clone()),
            
            ];
        let content = Content::from_lines(lines);

        assert_eq!(content.lines[0].index, first_index);
        assert_eq!(content.lines[0].text, first_text);
        assert_eq!(content.lines[1].index, second_index);
        assert_eq!(content.lines[1].text, second_text);
    }

    #[test]
    fn content_from_str_extracts_index_and_text() {
        let text = "Hello World!\nBy World!\nI like Rust :)";

        let content = Content::from_str(text);

        assert_eq!(content.lines[0].index, 1);
        assert_eq!(content.lines[0].text, "Hello World!");
        assert_eq!(content.lines[1].index, 2);
        assert_eq!(content.lines[1].text, "By World!");
        assert_eq!(content.lines[2].text, "I like Rust :)");
    }

    #[test]
    fn line_new_preserves_index_and_text() {
        let index = 39;
        let text = String::from("Hello World!");

        let line = Line::new(index, text.clone());

        assert_eq!(line.index, 39);
        assert_eq!(line.text, text);
    }
}