use crate::equation::expr::Expr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    right: Box<Expr>,
    op: UnaryOp,
}

impl UnaryExpr {
    pub fn new(right: Box<Expr>, op: UnaryOp) -> Self {
        Self { right, op }
    }

    pub fn right(&self) -> &Expr {
        self.right.as_ref()
    }

    pub fn op(&self) -> UnaryOp {
        self.op
    }
}
