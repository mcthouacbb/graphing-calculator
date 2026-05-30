use std::f64;

use crate::{
    equation::{expr::Expr, resolver::resolve_error::ResolveError},
    interval::Interval,
};

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
        Expr::Func(func_expr) => {
            resolve_builtin_constants(func_expr.input_mut());
        }
        Expr::ConstPowExpr(const_pow_expr) => {
            resolve_builtin_constants(const_pow_expr.base_mut());
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

const BUILTIN_FUNCS: [(&str, fn(f64) -> f64, fn(&Interval) -> Interval); 4] = [
    ("sin", f64::sin, Interval::sin),
    ("cos", f64::cos, Interval::cos),
    ("tan", f64::tan, Interval::tan),
    ("ln", f64::ln, Interval::ln),
];

pub fn resolve_builtin_functions(expr: &mut Expr) -> Result<(), ResolveError> {
    match expr {
        Expr::Binary(binary_expr) => {
            resolve_builtin_functions(binary_expr.left_mut())?;
            resolve_builtin_functions(binary_expr.right_mut())?;
        }
        Expr::Unary(unary_expr) => {
            resolve_builtin_functions(unary_expr.right_mut())?;
        }
        Expr::Func(func_expr) => {
            resolve_builtin_functions(func_expr.input_mut())?;
            for &(name, func, interval_func) in &BUILTIN_FUNCS {
                if func_expr.name() == name {
                    func_expr.set_func(func, interval_func);
                    return Ok(());
                }
            }
            return Err(ResolveError::UnknownFunction(func_expr.name().to_string()));
        }
        _ => (),
    }
    Ok(())
}
