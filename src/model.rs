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