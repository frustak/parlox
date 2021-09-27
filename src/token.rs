use crate::token_kind::TokenKind;
use std::fmt::Debug;

#[derive(Debug)]
pub struct GenericToken<T: Debug> {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: T,
    pub line: u32,
}

impl<T: Debug> GenericToken<T> {
    pub fn new(kind: TokenKind, lexeme: String, literal: T, line: u32) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

pub type Debuggable = Box<dyn Debug>;
pub type Token = GenericToken<Option<Debuggable>>;
