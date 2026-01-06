use crate::{
    expr::{Binary, Expr, Grouping, Literal, LiteralValue, Unary},
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub token: Token,
}

impl ParserError {
    pub fn new(message: &str, line: usize, token: Token) -> Self {
        Self {
            message: message.to_string(),
            line,
            token,
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual])
        {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;

            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.peek();

        match token.token_type {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralValue::Boolean(false),
                }))
            }

            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralValue::Boolean(true),
                }))
            }

            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralValue::Nil,
                }))
            }

            TokenType::Number => {
                let token = self.advance();
                let num = token.lexeme.parse::<f64>().map_err(|_| {
                    ParserError::new(
                        "Invalid number",
                        token.line as usize,
                        token.clone(),
                    )
                })?;
                Ok(Expr::Literal(Literal {
                    value: LiteralValue::Number(num),
                }))
            }

            TokenType::String => {
                let token = self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralValue::String(token.lexeme.clone()),
                }))
            }

            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;

                if self.peek().token_type != TokenType::RightParen {
                    let token = self.peek().clone();
                    return Err(ParserError::new(
                        "Expect ')' after expression",
                        token.line as usize,
                        token,
                    ));
                }
                self.advance();

                Ok(Expr::Grouping(Grouping {
                    expression: Box::new(expr),
                }))
            }

            _ => {
                let token = self.peek().clone();
                Err(ParserError::new(
                    "Expect expression",
                    token.line as usize,
                    token,
                ))
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for (_, t) in types.iter().enumerate() {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }
}
