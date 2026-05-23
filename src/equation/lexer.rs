use std::{
    char,
    iter::{Enumerate, Peekable},
    str::CharIndices,
};

use crate::equation::lexer::{
    lex_error::LexError,
    token::{Token, TokenKind},
};

pub mod lex_error;
pub mod token;

struct Lexer<'a> {
    source: &'a str,
    start: Peekable<CharIndices<'a>>,
    start_pos: u32,
    curr: Peekable<CharIndices<'a>>,
    curr_pos: u32,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            start: source.char_indices().peekable(),
            start_pos: 0,
            curr: source.char_indices().peekable(),
            curr_pos: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.curr.peek().map(|&(_, chr)| chr)
    }

    fn advance(&mut self) -> Option<char> {
        self.curr_pos += 1;
        self.curr.next().map(|(_, chr)| chr)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|chr| chr.is_whitespace()) {
            self.advance();
        }
    }

    fn get_token_data(&mut self) -> (&'a str, (u32, u32), (u32, u32)) {
        let start_idx = self.start.peek().unwrap().0;
        let curr_idx = self
            .curr
            .peek()
            .map(|&(idx, _)| idx)
            .unwrap_or(self.source.len());

        (
            &self.source[start_idx..curr_idx],
            (self.start_pos, self.curr_pos),
            (start_idx as u32, curr_idx as u32),
        )
    }

    fn next_token(&mut self) -> Result<Token<'a>, LexError> {
        self.skip_whitespace();

        self.start = self.curr.clone();
        self.start_pos = self.curr_pos;

        let peek = if let Some(chr) = self.advance() {
            chr
        } else {
            return Err(LexError::Eof);
        };

        let kind = if peek.is_ascii_digit() {
            while self.peek().is_some_and(|chr| chr.is_ascii_digit()) {
                self.advance();
            }

            // decimals
            if self.peek().is_some_and(|chr| chr == '.') {
                self.advance();
                while self.peek().is_some_and(|chr| chr.is_ascii_digit()) {
                    self.advance();
                }
            }

            // scientific notation
            if self.peek().is_some_and(|chr| chr == 'e') {
                self.advance();
                if self.peek().is_some_and(|chr| chr == '-') {
                    self.advance();
                }

                while self.peek().is_some_and(|chr| chr.is_ascii_digit()) {
                    self.advance();
                }
            }

            TokenKind::Literal
        } else if peek.is_alphabetic() {
            while self.peek().is_some_and(|chr| chr.is_ascii_alphanumeric()) {
                self.advance();
            }

            TokenKind::Identifier
        } else {
            match peek {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '^' => TokenKind::Caret,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                _ => return Err(LexError::InvalidToken(self.start_pos)),
            }
        };

        let (str, positions, indices) = self.get_token_data();
        Ok(Token::new(kind, str, positions, indices))
    }
}

pub fn get_tokens<'a>(str: &'a str) -> Result<Vec<Token<'a>>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(str);
    loop {
        match lexer.next_token() {
            Ok(token) => tokens.push(token),
            Err(err) => {
                if err == LexError::Eof {
                    break;
                } else {
                    return Err(err);
                }
            }
        }
    }

    Ok(tokens)
}
