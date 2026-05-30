use crate::{
    equation::expr::{Expr, binary_expr::BinaryOp, unary_expr::UnaryOp},
    interval::Interval,
};

pub struct ExplicitEquation {
    expr: Expr,
}

impl ExplicitEquation {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn calc(&self, x: f64) -> f64 {
        Self::calc_impl(&self.expr, x)
    }

    pub fn calc_interval(&self, interval: &Interval) -> Interval {
        Self::calc_interval_impl(&self.expr, interval)
    }

    fn calc_impl(expr: &Expr, x: f64) -> f64 {
        match expr {
            Expr::Binary(binary_expr) => {
                let left = Self::calc_impl(binary_expr.left(), x);
                let right = Self::calc_impl(binary_expr.right(), x);
                match binary_expr.op() {
                    BinaryOp::Add => left + right,
                    BinaryOp::Sub => left - right,
                    BinaryOp::Mul => left * right,
                    BinaryOp::Div => left / right,
                    BinaryOp::Pow => left.powf(right),
                }
            }
            Expr::Unary(unary_expr) => {
                let right = Self::calc_impl(unary_expr.right(), x);
                match unary_expr.op() {
                    UnaryOp::Neg => -right,
                }
            }
            Expr::Func(func_expr) => {
                let input = Self::calc_impl(func_expr.input(), x);
                func_expr.func()(input)
            }
            Expr::Const(const_expr) => const_expr.value(),
            Expr::Var(var_expr) => {
                assert!(var_expr.name() == "x");
                x
            }
        }
    }

    fn calc_interval_impl(expr: &Expr, interval: &Interval) -> Interval {
        match expr {
            Expr::Binary(binary_expr) => {
                let left = Self::calc_interval_impl(binary_expr.left(), interval);
                let right = Self::calc_interval_impl(binary_expr.right(), interval);
                match binary_expr.op() {
                    BinaryOp::Add => left + right,
                    BinaryOp::Sub => left - right,
                    BinaryOp::Mul => left * right,
                    BinaryOp::Div => left / right,
                    BinaryOp::Pow => left.pow(&right),
                }
            }
            Expr::Unary(unary_expr) => {
                let right = Self::calc_interval_impl(unary_expr.right(), interval);
                match unary_expr.op() {
                    UnaryOp::Neg => -right,
                }
            }
            Expr::Func(func_expr) => {
                let input = Self::calc_interval_impl(func_expr.input(), interval);
                func_expr.interval_func()(&input)
            }
            Expr::Const(const_expr) => Interval::new(const_expr.value(), const_expr.value()),
            Expr::Var(var_expr) => {
                assert!(var_expr.name() == "x");
                *interval
            }
        }
    }
}
