use crate::equation::{
    Equation, expr::Expr, parser::ExprOrEquation, resolver::resolve_error::ResolveError,
};

pub mod resolve_error;

pub fn check_identifiers(expr: &Expr) -> Result<(), ResolveError> {
    match expr {
        Expr::Binary(binary_expr) => {
            check_identifiers(binary_expr.left())?;
            check_identifiers(binary_expr.right())?;
        }
        Expr::Unary(unary_expr) => {
            check_identifiers(unary_expr.right())?;
        }
        Expr::Var(var_expr) => {
            if var_expr.name() != "x" && var_expr.name() != "y" {
                return Err(ResolveError::UnknownIdentifier(var_expr.name().to_string()));
            }
        }
        Expr::Const(_) => (),
    }
    Ok(())
}

pub fn resolve_equation(expr_or_equation: ExprOrEquation) -> Result<Equation, ResolveError> {
    match expr_or_equation {
        ExprOrEquation::Expr(expr) => {
            check_identifiers(&expr)?;
            Ok(Equation {
                left: expr,
                right: Expr::new_var("y".to_owned()),
            })
        }
        ExprOrEquation::Equation(left, right) => {
            check_identifiers(&left)?;
            check_identifiers(&right)?;
            Ok(Equation { left, right })
        }
    }
}
