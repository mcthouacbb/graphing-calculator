use std::f64;

use crate::equation::expr::Expr;

const BUILTIN_CONSTS: [(&str, f64); 2] = [("e", f64::consts::E), ("pi", f64::consts::PI)];

pub fn resolve_builtin_constants(expr: &mut Expr) {
    match expr {
        Expr::Binary(binary_expr) => {
            resolve_builtin_constants(binary_expr.left_mut());
            resolve_builtin_constants(binary_expr.right_mut());
        }
        Expr::Unary(unary_expr) => {
            resolve_builtin_constants(unary_expr.right_mut());
        }
        Expr::Var(var_expr) => {
            for &(name, value) in &BUILTIN_CONSTS {
                if var_expr.name() == name {
                    *expr = Expr::new_const(value);
                    return;
                }
            }
        }
        Expr::Const(_) => (),
    }
}
