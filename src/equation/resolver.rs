use crate::equation::{
    Equation,
    expr::Expr,
    parser::ExprOrEquation,
    resolver::{builtins::resolve_builtin_constants, resolve_error::ResolveError},
};

mod builtins;
pub mod resolve_error;

pub fn check_identifiers(expr: &Expr, expected: &[&str]) -> Result<(), ResolveError> {
    match expr {
        Expr::Binary(binary_expr) => {
            check_identifiers(binary_expr.left(), expected)?;
            check_identifiers(binary_expr.right(), expected)?;
        }
        Expr::Unary(unary_expr) => {
            check_identifiers(unary_expr.right(), expected)?;
        }
        Expr::Var(var_expr) => {
            for identifier in expected {
                if var_expr.name() == *identifier {
                    return Ok(());
                }
            }
            return Err(ResolveError::UnknownIdentifier(var_expr.name().to_string()));
        }
        Expr::Const(_) => (),
    }
    Ok(())
}

pub fn resolve_equation(expr_or_equation: ExprOrEquation) -> Result<Equation, ResolveError> {
    match expr_or_equation {
        ExprOrEquation::Expr(mut expr) => {
            resolve_builtin_constants(&mut expr);

            if check_identifiers(&expr, &["x"]).is_err() {
                return Err(ResolveError::IncompleteEquation);
            }

            Ok(Equation::new_explicit(expr))
        }
        ExprOrEquation::Equation(mut left, mut right) => {
            resolve_builtin_constants(&mut left);
            resolve_builtin_constants(&mut right);

            check_identifiers(&left, &["x", "y"])?;
            check_identifiers(&right, &["x", "y"])?;

            if let Expr::Var(left_var) = &left
                && left_var.name() == "y"
                && check_identifiers(&right, &["x"]).is_ok()
            {
                return Ok(Equation::new_explicit(right));
            }

            if let Expr::Var(right_var) = &right
                && right_var.name() == "y"
                && check_identifiers(&left, &["x"]).is_ok()
            {
                return Ok(Equation::new_explicit(left));
            }
            Err(ResolveError::UnsupportedEquation)
        }
    }
}
