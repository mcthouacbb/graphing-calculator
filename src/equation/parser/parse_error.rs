use crate::equation::lexer::token::{Token, TokenKind};

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'a> {
    ExpectedPrimary(Option<Token<'a>>),
    ExpectedToken(TokenKind, Option<Token<'a>>),
    ExtraToken(Token<'a>),
}
