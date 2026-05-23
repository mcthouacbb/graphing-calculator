use crate::equation::expr::{Expr, binary_expr::BinaryOp, unary_expr::UnaryOp};

mod expr;
pub mod lexer;
pub mod parser;
pub mod resolver;

pub struct Equation {
    left: Expr,
    right: Expr,
}

impl Equation {
    fn calc_residual_impl(expr: &Expr, x: f64, y: f64) -> f64 {
        match expr {
            Expr::Binary(binary_expr) => {
                let left_val = Self::calc_residual_impl(binary_expr.left(), x, y);
                let right_val = Self::calc_residual_impl(binary_expr.right(), x, y);
                match binary_expr.op() {
                    BinaryOp::Add => left_val + right_val,
                    BinaryOp::Sub => left_val - right_val,
                    BinaryOp::Mul => left_val * right_val,
                    BinaryOp::Div => left_val / right_val,
                    BinaryOp::Pow => left_val.powf(right_val),
                }
            }
            Expr::Unary(unary_expr) => {
                let right_val = Self::calc_residual_impl(unary_expr.right(), x, y);
                match unary_expr.op() {
                    UnaryOp::Neg => -right_val,
                }
            }
            Expr::Var(var_expr) => {
                if var_expr.name() == "x" {
                    x
                } else if var_expr.name() == "y" {
                    y
                } else {
                    panic!("Variable other than x or y in Equation::calc_residual()");
                }
            }
            Expr::Const(const_expr) => const_expr.value(),
        }
    }

    pub fn calc_residual(&self, x: f64, y: f64) -> f64 {
        Self::calc_residual_impl(&self.left, x, y) - Self::calc_residual_impl(&self.right, x, y)
    }
}
