use crate::expr::Expr;
use crate::token::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum ParseError {
    UnbalancedParens(Token, String),
    UnknownPrimary(Token, String),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;

        while self.token_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.term()?;

        while self.token_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.factor()?;

        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;

        while self.token_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.token_match(&[TokenType::False]) {
            return Ok(Box::new(Expr::Literal(Literal::Boolean(false))));
        }
        if self.token_match(&[TokenType::True]) {
            return Ok(Box::new(Expr::Literal(Literal::Boolean(true))));
        }
        if self.token_match(&[TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(Literal::Nil)));
        }

        if self.token_match(&[TokenType::Number, TokenType::LoxString]) {
            return Ok(Box::new(Expr::Literal(self.previous().literal())));
        }

        if self.token_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            let consume_result = self.consume(
                TokenType::RightParen,
                String::from("Expect ')' after expression."),
            );
            if let Err(e) = consume_result {
                return Err(e);
            }
            return Ok(Box::new(Expr::Grouping(expr)));
        }
        Err(ParseError::UnknownPrimary(
            self.peek(),
            String::from("Expected expression"),
        ))
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParseError::UnbalancedParens(self.peek(), message))
    }

    fn token_match(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek().token_type() == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type() == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type() == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => self.peek(),
                _ => self.advance(),
            };
        }
    }
}
