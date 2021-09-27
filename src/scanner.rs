use crate::{
    token::{Debuggable, Token},
    token_kind::{TokenKind, KEYWORDS},
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct SyntaxError {
    line: u32,
    message: String,
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<SyntaxError>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<SyntaxError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let token = Token::new(TokenKind::Eof, "".to_string(), None, self.line);
        self.tokens.push(token);

        if !self.errors.is_empty() {
            Err(self.errors)
        } else {
            Ok(self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        use TokenKind::*;
        let character = self.advance();

        match character {
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(LeftBrace, None),
            '}' => self.add_token(RightBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),
            '!' if self.matches('=') => self.add_token(BangEqual, None),
            '!' => self.add_token(Bang, None),
            '=' if self.matches('=') => self.add_token(EqualEqual, None),
            '=' => self.add_token(Equal, None),
            '<' if self.matches('=') => self.add_token(LessEqual, None),
            '<' => self.add_token(Less, None),
            '>' if self.matches('=') => self.add_token(GreaterEqual, None),
            '>' => self.add_token(Greater, None),
            '/' if self.matches('/') => self.advance_line(),
            '/' => self.add_token(Slash, None),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            letter if letter.is_digit(10) => self.number(),
            letter if letter.is_alphabetic() || letter == '_' => self.identifier(),
            _ => self.errors.push(SyntaxError {
                line: self.line,
                message: "Unexpected Character.".to_string(),
            }),
        };
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance_line(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(SyntaxError {
                line: self.line,
                message: "Unterminated string.".to_string(),
            });
            return;
        }

        self.advance();
        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenKind::String, Some(Box::new(literal)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenKind::Number, Some(Box::new(literal)));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).unwrap_or(&TokenKind::Identifier);
        self.add_token(*token_type, None);
    }

    fn add_token(&mut self, token_type: TokenKind, literal: Option<Debuggable>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }
}
