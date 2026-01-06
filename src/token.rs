use std::fmt;

use crate::token_type::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType, // type 대신 token_type
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"null".to_string())
        )
    }
}
