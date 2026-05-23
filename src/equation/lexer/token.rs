use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Literal,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    kind: TokenKind,
    str: &'a str,
    positions: (u32, u32),
    indices: (u32, u32),
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, str: &'a str, positions: (u32, u32), indices: (u32, u32)) -> Self {
        Self {
            kind,
            str,
            positions,
            indices,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn str(&self) -> &str {
        self.str
    }

    pub fn positions(&self) -> (u32, u32) {
        self.positions
    }

    pub fn indices(&self) -> (u32, u32) {
        self.indices
    }
}
