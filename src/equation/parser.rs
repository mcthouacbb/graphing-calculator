use crate::equation::{
    expr::{Expr, binary_expr::BinaryOp, unary_expr::UnaryOp},
    lexer::token::{Token, TokenKind},
    parser::parse_error::ParseError,
};

pub mod parse_error;

pub enum ExprOrEquation {
    Expr(Expr),
    Equation(Expr, Expr),
}

struct Parser<'a, 'b> {
    tokens: &'b [Token<'a>],
    curr: usize,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn new(tokens: &'b [Token<'a>]) -> Self {
        Self { tokens, curr: 0 }
    }

    fn peek_nth(&self, n: i32) -> Option<Token<'a>> {
        self.tokens.get(self.curr.wrapping_add(n as usize)).cloned()
    }

    fn peek(&self) -> Option<Token<'a>> {
        self.peek_nth(0)
    }

    fn peek_prev(&self) -> Option<Token<'a>> {
        self.peek_nth(-1)
    }

    fn advance(&mut self) -> Token<'a> {
        let tok = self
            .peek()
            .expect("Attempting to advance more than one past the end");
        self.curr += 1;
        tok
    }

    fn match_tok(&mut self, kind: TokenKind) -> bool {
        if self.peek().is_some_and(|tok| tok.kind() == kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError<'a>> {
        if self.match_tok(kind) {
            Ok(())
        } else {
            Err(ParseError::ExpectedToken(kind, self.peek()))
        }
    }

    fn parse(&mut self) -> Result<ExprOrEquation, ParseError<'a>> {
        let left = self.parse_add_sub()?;
        if self.match_tok(TokenKind::Equal) {
            let right = self.parse_add_sub()?;
            if let Some(token) = self.peek() {
                Err(ParseError::ExtraToken(token))
            } else {
                Ok(ExprOrEquation::Equation(left, right))
            }
        } else if let Some(token) = self.peek() {
            Err(ParseError::ExtraToken(token))
        } else {
            Ok(ExprOrEquation::Expr(left))
        }
    }

    fn parse_add_sub(&mut self) -> Result<Expr, ParseError<'a>> {
        let mut result = self.parse_mul_div()?;
        loop {
            let op = if self.match_tok(TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_tok(TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.parse_mul_div()?;

            result = Expr::new_binary(Box::new(result), Box::new(right), op);
        }

        Ok(result)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, ParseError<'a>> {
        let mut result = self.parse_pow()?;
        loop {
            let op = if self.match_tok(TokenKind::Star) {
                BinaryOp::Mul
            } else if self.match_tok(TokenKind::Slash) {
                BinaryOp::Div
            } else {
                break;
            };

            let right = self.parse_pow()?;

            result = Expr::new_binary(Box::new(result), Box::new(right), op);
        }

        Ok(result)
    }

    fn parse_pow(&mut self) -> Result<Expr, ParseError<'a>> {
        let result = self.parse_neg()?;
        if self.match_tok(TokenKind::Caret) {
            let right = self.parse_pow()?;
            Ok(Expr::new_binary(
                Box::new(result),
                Box::new(right),
                BinaryOp::Pow,
            ))
        } else {
            Ok(result)
        }
    }

    fn parse_neg(&mut self) -> Result<Expr, ParseError<'a>> {
        if self.match_tok(TokenKind::Minus) {
            let right = self.parse_neg()?;
            Ok(Expr::new_unary(Box::new(right), UnaryOp::Neg))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError<'a>> {
        if self.match_tok(TokenKind::Literal) {
            let tok = self.peek_prev().unwrap();
            let value = tok
                .str()
                .parse::<f64>()
                .expect("Valid literal token can't be parsed");

            Ok(Expr::new_const(value))
        } else if self.match_tok(TokenKind::Identifier) {
            let tok = self.peek_prev().unwrap();
            let name = tok.str().to_string();

            Ok(Expr::new_var(name))
        } else if self.match_tok(TokenKind::OpenParen) {
            let expr = self.parse_add_sub()?;
            self.expect(TokenKind::CloseParen)?;

            Ok(expr)
        } else {
            Err(ParseError::ExpectedPrimary(self.peek()))
        }
    }
}

pub fn parse_expr_or_equation<'a>(
    tokens: &Vec<Token<'a>>,
) -> Result<ExprOrEquation, ParseError<'a>> {
    let mut parser = Parser::new(&tokens);
    parser.parse()
}
