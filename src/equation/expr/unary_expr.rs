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

    pub fn right_mut(&mut self) -> &mut Expr {
        self.right.as_mut()
    }

    pub fn op(&self) -> UnaryOp {
        self.op
    }
}
