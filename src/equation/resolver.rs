use crate::equation::{
    Equation, expr::Expr, parser::ExprOrEquation, resolver::resolve_error::ResolveError,
};

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
        ExprOrEquation::Expr(expr) => {
            if check_identifiers(&expr, &["x"]).is_err() {
                return Err(ResolveError::IncompleteEquation);
            }

            Ok(Equation {
                left: expr,
                right: Expr::new_var("y".to_owned()),
            })
        }
        ExprOrEquation::Equation(left, right) => {
            check_identifiers(&left, &["x", "y"])?;
            check_identifiers(&right, &["x", "y"])?;
            Ok(Equation { left, right })
        }
    }
}
