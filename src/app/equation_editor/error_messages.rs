use crate::equation::{
    lexer::{lex_error::LexError, token::TokenKind},
    parser::parse_error::ParseError,
    resolver::resolve_error::ResolveError,
};

pub fn lexer_error_message(lex_error: &LexError) -> String {
    match lex_error {
        LexError::InvalidToken(pos, chr) => {
            format!("Unexpected character '{}' at position {}.", chr, pos)
        }
        LexError::Eof => panic!("Unexpected LexError::Eof"),
    }
}

pub fn parser_error_message(parse_error: &ParseError) -> String {
    match parse_error {
        ParseError::ExpectedPrimary(actual) => {
            if let Some(token) = actual {
                // TODO: add function call
                format!(
                    "Expected a number, variable, or parenthesized expression at position {}. Instead got '{}'",
                    token.positions().0,
                    token.str()
                )
            } else {
                format!(
                    "Expected a number, variable, or parenthesized expression. Instead got end of input"
                )
            }
        }
        ParseError::ExpectedToken(kind, actual) => {
            let kind_str = match kind {
                // TODO: add function call
                TokenKind::Identifier => "a variable",
                TokenKind::Literal => "a constant",
                TokenKind::Plus => "'+'",
                TokenKind::Minus => "'-'",
                TokenKind::Star => "'*'",
                TokenKind::Slash => "'/'",
                TokenKind::Caret => "'^'",
                TokenKind::Equal => "'='",
                TokenKind::OpenParen => "'('",
                TokenKind::CloseParen => "')'",
            };

            if let Some(token) = actual {
                format!(
                    "Expected {} at position {}. Instead got '{}'.",
                    kind_str,
                    token.positions().0,
                    token.str()
                )
            } else {
                format!("Expected {}. Instead got end of input.", kind_str)
            }
        }
        ParseError::ExtraToken(token) => {
            format!(
                "Unexpected extra token '{}' at position {}.",
                token.str(),
                token.positions().0
            )
        }
    }
}

pub fn resolver_error_message(resolve_error: &ResolveError) -> String {
    match resolve_error {
        ResolveError::UnknownIdentifier(identifier) => {
            // TODO: track where the identifier came from
            format!("Unknown identifier '{}'", identifier)
        }
        ResolveError::IncompleteEquation => {
            format!("Incomplete equation")
        }
    }
}
