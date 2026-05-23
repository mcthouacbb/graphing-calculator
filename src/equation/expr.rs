use crate::equation::expr::{
    binary_expr::{BinaryExpr, BinaryOp},
    const_expr::ConstExpr,
    unary_expr::{UnaryExpr, UnaryOp},
    var_expr::VarExpr,
};

pub mod binary_expr;
pub mod const_expr;
pub mod unary_expr;
pub mod var_expr;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Var(VarExpr),
    Const(ConstExpr),
}

impl Expr {
    pub fn new_binary(left: Box<Expr>, right: Box<Expr>, op: BinaryOp) -> Self {
        Self::Binary(BinaryExpr::new(left, right, op))
    }

    pub fn new_unary(right: Box<Expr>, op: UnaryOp) -> Self {
        Self::Unary(UnaryExpr::new(right, op))
    }

    pub fn new_var(name: String) -> Self {
        Self::Var(VarExpr::new(name))
    }

    pub fn new_const(value: f64) -> Self {
        Self::Const(ConstExpr::new(value))
    }
}
