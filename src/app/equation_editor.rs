use eframe::egui;

use crate::equation::{
    Equation,
    lexer::{get_tokens, lex_error::LexError, token::TokenKind},
    parser::{parse_error::ParseError, parse_expr_or_equation},
    resolver::{resolve_equation, resolve_error::ResolveError},
};

pub struct EquationEditor {
    data: String,
    equation: Option<Equation>,
    error: Option<String>,
}

impl EquationEditor {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            equation: None,
            error: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.text_edit_field(ui);
        if let Some(error) = &self.error {
            ui.label(error);
        }
    }

    pub fn equation(&self) -> &Option<Equation> {
        &self.equation
    }

    pub fn data(&self) -> &str {
        self.data.as_str()
    }

    fn text_edit_field(&mut self, ui: &mut egui::Ui) {
        if ui.text_edit_singleline(&mut self.data).changed() {
            if self.data.trim().is_empty() {
                self.equation = None;
                self.error = None;
                return;
            }

            let lex_result = get_tokens(&self.data);
            if let Err(lex_error) = &lex_result {
                self.equation = None;

                self.error = Some(match lex_error {
                    LexError::InvalidToken(pos, chr) => {
                        format!("Unexpected character '{}' at position {}.", chr, pos)
                    }
                    LexError::Eof => panic!("Unexpected LexError::Eof"),
                });
                return;
            }

            let tokens = lex_result.unwrap();
            let parse_result = parse_expr_or_equation(&tokens);
            if let Err(parse_error) = &parse_result {
                self.equation = None;

                self.error = Some(match parse_error {
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
                });
                return;
            }

            let resolve_result = resolve_equation(parse_result.unwrap());
            if let Err(resolve_err) = &resolve_result {
                self.equation = None;

                self.error = Some(match resolve_err {
                    ResolveError::UnknownIdentifier(identifier) => {
                        // TODO: track where the identifier came from
                        format!("Unknown identifier '{}'", identifier)
                    }
                });
                return;
            }

            self.equation = Some(resolve_result.unwrap());
            self.error = None;
        }
    }
}
