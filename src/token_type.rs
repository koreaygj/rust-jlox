#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // 단일 문자 토큰
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // 문자 1개 또는 2개 짜리 토큰
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // 리터럴
    Identifier,
    String,
    Number,

    // 키워드
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
