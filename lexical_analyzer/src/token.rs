use crate::double_buffer::{DoubleFixedBuffer, DoubleFixedBufferCursor};
use std::io::Read;

const IDENTIFIER: &str = "id";

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: String, // This could be converted to a string reference to conserve space
    pub error_token: bool,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new<T: Read>(
        keywords: &Vec<String>,
        token_type: &String,
        error_token: bool,
        source: &DoubleFixedBuffer<T>,
        start: &DoubleFixedBufferCursor,
        offset: usize,
        line: usize,
        column: usize,
    ) -> Self {
        let lexeme = std::str::from_utf8(&start.copy_bytes(offset, source))
            .expect("lexeme is invalid UTF-8")
            .to_string();
        let token_type = if token_type == IDENTIFIER && keywords.contains(&lexeme) {
            lexeme.clone()
        } else {
            token_type.clone()
        };

        Token {
            token_type,
            error_token,
            lexeme,
            line,
            column,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // replace all \r and \n character with their respective escapes
        let escaped_lexeme = self.lexeme.replace("\n", "\\n").replace("\r", "\\r");
        write!(
            f,
            "[{}, {}, {}:{}]",
            self.token_type, escaped_lexeme, self.line, self.column
        )
    }
}
