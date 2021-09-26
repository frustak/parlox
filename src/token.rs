use crate::token_kind::TokenKind;
use std::any::Any;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Box<dyn Any>>,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Option<Box<dyn Any>>, line: u32) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}
