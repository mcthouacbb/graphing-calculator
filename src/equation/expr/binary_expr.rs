use crate::equation::expr::Expr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    op: BinaryOp,
}

impl BinaryExpr {
    pub fn new(left: Box<Expr>, right: Box<Expr>, op: BinaryOp) -> Self {
        Self { left, right, op }
    }

    pub fn left(&self) -> &Expr {
        self.left.as_ref()
    }

    pub fn right(&self) -> &Expr {
        self.right.as_ref()
    }

    pub fn left_mut(&mut self) -> &mut Expr {
        self.left.as_mut()
    }

    pub fn right_mut(&mut self) -> &mut Expr {
        self.right.as_mut()
    }

    pub fn op(&self) -> BinaryOp {
        self.op
    }
}
