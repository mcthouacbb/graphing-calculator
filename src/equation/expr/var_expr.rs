#[derive(Debug, Clone)]
pub struct VarExpr {
    name: String,
}

impl VarExpr {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
