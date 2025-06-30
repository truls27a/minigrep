use crate::model::{Content, Line};

impl Content {
    pub fn display(&self, show_line_numbers: bool) {
        for line in &self.lines {
            line.display(show_line_numbers);
        }
    }
}

impl Line {
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