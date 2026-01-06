use std::collections::HashMap;

use crate::{token::Token, token_type::TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
    had_error: bool,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        // ✅ HashMap 초기화
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);
        Scanner {
            source,
            tokens: Vec::new(),
            chars,
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // 단일 문자 토큰
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // 주석: 줄 끝까지
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            // 공백 무시
            ' ' | '\r' | '\t' => {}

            // 줄바꿈
            '\n' => {
                self.line += 1;
            }
            '"' => self.scan_string(),

            _ => {
                if c.is_ascii_digit() {
                    self.scan_number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.scan_identifier();
                } else {
                    self.error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<String>,
    ) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        });
    }

    // String literal
    fn scan_string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::String, Some(value.to_string()));
    }

    //scan number
    fn scan_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        self.add_token_literal(TokenType::Number, Some(value.to_string()));
    }

    //scan identifier
    fn scan_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type = self
            .keywords
            .get(text)
            .copied() // &TokenType -> TokenType
            .unwrap_or(TokenType::Identifier); // 없으면 Identifier

        self.add_token(token_type);
    }

    // Error
    fn error(&mut self, line: i32, message: &str) {
        eprintln!("[line {}] Error: {}", self.line, message);
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_character_tokens() {
        let source = "(){},.-+;*".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::RightParen);
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::Dot);
        assert_eq!(tokens[6].token_type, TokenType::Minus);
        assert_eq!(tokens[7].token_type, TokenType::Plus);
        assert_eq!(tokens[8].token_type, TokenType::Semicolon);
        assert_eq!(tokens[9].token_type, TokenType::Star);
        assert_eq!(tokens[10].token_type, TokenType::Eof);
    }

    #[test]
    fn test_two_character_tokens() {
        let source = "! != = == < <= > >=".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[1].token_type, TokenType::BangEqual);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[4].token_type, TokenType::Less);
        assert_eq!(tokens[5].token_type, TokenType::LessEqual);
        assert_eq!(tokens[6].token_type, TokenType::Greater);
        assert_eq!(tokens[7].token_type, TokenType::GreaterEqual);
    }

    #[test]
    fn test_slash_and_comment() {
        let source = "/ // this is a comment\n/".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[1].token_type, TokenType::Slash);
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_string_literal() {
        let source = r#""hello world""#.to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].literal, Some("hello world".to_string()));
        assert_eq!(tokens[0].lexeme, r#""hello world""#);
    }

    #[test]
    fn test_multiline_string() {
        let source = "\"hello\nworld\"".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].line, 2);
    }

    #[test]
    fn test_number_integer() {
        let source = "123".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some("123".to_string()));
    }

    #[test]
    fn test_number_decimal() {
        let source = "123.456".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some("123.456".to_string()));
    }

    #[test]
    fn test_keywords() {
        let source = "if else while for class fun var".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(tokens[1].token_type, TokenType::Else);
        assert_eq!(tokens[2].token_type, TokenType::While);
        assert_eq!(tokens[3].token_type, TokenType::For);
        assert_eq!(tokens[4].token_type, TokenType::Class);
        assert_eq!(tokens[5].token_type, TokenType::Fun);
        assert_eq!(tokens[6].token_type, TokenType::Var);
    }

    #[test]
    fn test_identifiers() {
        let source = "myVar _test123 hello_world".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "myVar");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "_test123");
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].lexeme, "hello_world");
    }

    #[test]
    fn test_whitespace_handling() {
        let source = "  \t\r\n  var  \n  x  ".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_complete_statement() {
        let source = "var x = 42;".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "x");
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].literal, Some("42".to_string()));
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::Eof);
    }

    #[test]
    fn test_expression() {
        let source = "3 + 4 * 5 - 2".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[3].token_type, TokenType::Star);
        assert_eq!(tokens[4].token_type, TokenType::Number);
        assert_eq!(tokens[5].token_type, TokenType::Minus);
        assert_eq!(tokens[6].token_type, TokenType::Number);
    }

    #[test]
    fn test_line_numbers() {
        let source = "var x\nvar y\nvar z".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[2].line, 2);
        assert_eq!(tokens[3].line, 2);
        assert_eq!(tokens[4].line, 3);
        assert_eq!(tokens[5].line, 3);
    }

    #[test]
    fn test_boolean_literals() {
        let source = "true false nil".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::True);
        assert_eq!(tokens[1].token_type, TokenType::False);
        assert_eq!(tokens[2].token_type, TokenType::Nil);
    }
}
