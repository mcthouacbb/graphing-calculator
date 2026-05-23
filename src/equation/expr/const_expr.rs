#[derive(Debug, Clone)]
pub struct ConstExpr {
    value: f64,
}

impl ConstExpr {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
