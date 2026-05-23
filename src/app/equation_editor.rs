use eframe::egui;

use crate::equation::{
    Equation, lexer::get_tokens, parser::parse_expr_or_equation, resolver::resolve_equation,
};

mod error_messages;

pub struct EquationEditor {
    data: String,
    equation: Option<Result<Equation, String>>,
}

impl EquationEditor {
    pub fn new() -> Self {
        Self {
            data: String::new(),
            equation: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.text_edit_field(ui);
        if let Some(error) = self.error() {
            ui.label(error);
        }
    }

    pub fn equation(&self) -> Option<&Equation> {
        self.equation
            .as_ref()
            .map(|result| result.as_ref().ok())
            .flatten()
    }

    pub fn error(&self) -> Option<&String> {
        self.equation
            .as_ref()
            .map(|result| result.as_ref().err())
            .flatten()
    }

    pub fn data(&self) -> &str {
        self.data.as_str()
    }

    fn text_edit_field(&mut self, ui: &mut egui::Ui) {
        if ui.text_edit_singleline(&mut self.data).changed() {
            if self.data.trim().is_empty() {
                self.equation = None;
                return;
            }

            let lex_result = get_tokens(&self.data);
            if let Err(lex_error) = &lex_result {
                let error_message = error_messages::lexer_error_message(lex_error);
                self.equation = Some(Err(error_message));
                return;
            }

            let tokens = lex_result.unwrap();
            let parse_result = parse_expr_or_equation(&tokens);
            if let Err(parse_error) = &parse_result {
                let error_message = error_messages::parser_error_message(parse_error);
                self.equation = Some(Err(error_message));
                return;
            }

            let resolve_result = resolve_equation(parse_result.unwrap());
            if let Err(resolve_error) = &resolve_result {
                let error_message = error_messages::resolver_error_message(resolve_error);
                self.equation = Some(Err(error_message));
                return;
            }

            self.equation = Some(Ok(resolve_result.unwrap()));
        }
    }
}
