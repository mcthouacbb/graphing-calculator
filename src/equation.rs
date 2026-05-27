use crate::equation::{explicit::ExplicitEquation, expr::Expr};

pub mod explicit;
mod expr;
pub mod lexer;
pub mod parser;
pub mod resolver;

pub enum Equation {
    Explicit(ExplicitEquation),
}

impl Equation {
    pub fn new_explicit(expr: Expr) -> Self {
        Self::Explicit(ExplicitEquation::new(expr))
    }
}
