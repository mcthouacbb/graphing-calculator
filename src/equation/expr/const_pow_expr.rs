use crate::equation::expr::Expr;

#[derive(Debug, Clone)]
pub struct ConstPowExpr {
    base: Box<Expr>,
    power: f64,
}

impl ConstPowExpr {
    pub fn new(base: Box<Expr>, power: f64) -> Self {
        Self { base, power }
    }

    pub fn base(&self) -> &Expr {
        self.base.as_ref()
    }

    pub fn base_mut(&mut self) -> &mut Expr {
        self.base.as_mut()
    }

    pub fn power(&self) -> f64 {
        self.power
    }
}
