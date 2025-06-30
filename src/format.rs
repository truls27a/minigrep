#[derive(Debug, PartialEq)]
pub struct Line {
    pub index: usize,
    pub content: String,
}

impl Line {
    pub fn new(index: usize, content: String) -> Line {
        Line { index, content }
    }

    pub fn highlight(&self, query: &str) -> String {
        let mut colored_content = String::new();

        let content_chars: Vec<char> = self.content.chars().collect();
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