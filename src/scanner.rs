use crate::token::{Literal, Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_owned(), TokenType::And);
        m.insert("class".to_owned(), TokenType::Class);
        m.insert("else".to_owned(), TokenType::Else);
        m.insert("false".to_owned(), TokenType::False);
        m.insert("for".to_owned(), TokenType::For);
        m.insert("fun".to_owned(), TokenType::Fun);
        m.insert("if".to_owned(), TokenType::If);
        m.insert("nil".to_owned(), TokenType::Nil);
        m.insert("or".to_owned(), TokenType::Or);
        m.insert("print".to_owned(), TokenType::Print);
        m.insert("return".to_owned(), TokenType::Return);
        m.insert("super".to_owned(), TokenType::Super);
        m.insert("this".to_owned(), TokenType::This);
        m.insert("true".to_owned(), TokenType::True);
        m.insert("var".to_owned(), TokenType::Var);
        m.insert("while".to_owned(), TokenType::While);
        m
    };
}

/// `ScannerError` is an enum of errors that can occur while scanning tokens.
pub enum ScannerError {
    /// An `UnexpectedChar` error occurs if the scanner encounters a byte that
    /// it does not know how to handle
    UnexpectedChar(usize),
    /// An `UnterminatedString` error occurs if the scanner finds an unterminated
    /// string literal.
    UnterminatedString(usize),
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    /// Create an empty `Scanner`.
    /// The `Scanner` will be populated with a `Vec<u8>` containing the result
    /// of converting `source` into bytes.
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.into_bytes(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// `scan_tokens` parses the bytes held by the `Scanner` and returns an immutable reference
    /// to a `Vec` of `Token`s.
    /// If an error occurs while scanning, it will be returned with the number of the line
    ///  on which the error was found.
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ScannerError> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        Ok(&self.tokens)
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'*' => self.add_token(TokenType::Star, None),
            b'!' => {
                if self.char_match(b'=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            b'=' => {
                if self.char_match(b'=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            b'<' => {
                if self.char_match(b'=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            b'>' => {
                if self.char_match(b'=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            b'/' => {
                if self.char_match(b'/') {
                    while self.peek() != b'\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            b'"' => {
                self.string()?;
            }
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            other => {
                if other.is_ascii_digit() {
                    self.number()?;
                } else if other.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    return Err(ScannerError::UnexpectedChar(self.line));
                }
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        let curr_char = self.source[self.current];
        self.current += 1;
        curr_char
    }

    fn peek(&self) -> u8 {
        if self.at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn string(&mut self) -> Result<(), ScannerError> {
        while self.peek() != b'"' && !self.at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            return Err(ScannerError::UnterminatedString(self.line));
        }
        self.advance();
        match self.substring(self.start + 1, self.current - 1) {
            Err(_) => {
                return Err(ScannerError::UnexpectedChar(self.line));
            }
            Ok(value) => self.add_token(TokenType::LoxString, Some(Literal::LoxString(value))),
        }
        Ok(())
    }

    fn number(&mut self) -> Result<(), ScannerError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        match self.substring(self.start, self.current) {
            Err(_) => {
                return Err(ScannerError::UnexpectedChar(self.line));
            }
            Ok(string_value) => {
                let float_value: Result<f64, _> = string_value.parse();
                // should always be the case since we are only reading valid numbers
                if let Ok(value) = float_value {
                    self.add_token(TokenType::Number, Some(Literal::Number(value)))
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        if let Ok(text) = self.substring(self.start, self.current) {
            if let Some(token_type) = KEYWORDS.get(&text) {
                self.add_token(*token_type, None);
            } else {
                self.add_token(TokenType::Identifier, None);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let buf = &self.source[self.start..self.current];
        let text = String::from_utf8_lossy(buf);
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    fn char_match(&mut self, expected: u8) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn substring(&self, start: usize, end: usize) -> Result<String, ScannerError> {
        let value = self.source[start..end].to_owned();
        let str_value = String::from_utf8(value);
        if str_value.is_err() {
            return Err(ScannerError::UnexpectedChar(self.line));
        }
        Ok(str_value.unwrap())
    }
}
