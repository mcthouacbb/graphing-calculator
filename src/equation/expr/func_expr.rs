use crate::{equation::expr::Expr, interval::Interval};

#[derive(Debug, Clone)]
pub struct FuncExpr {
    input: Box<Expr>,
    name: String,
    func: Option<fn(f64) -> f64>,
    interval_func: Option<fn(&Interval) -> Interval>,
}

impl FuncExpr {
    pub fn new(input: Box<Expr>, name: String) -> Self {
        Self {
            input,
            name,
            func: None,
            interval_func: None,
        }
    }

    pub fn input(&self) -> &Expr {
        self.input.as_ref()
    }

    pub fn input_mut(&mut self) -> &mut Expr {
        self.input.as_mut()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn func(&self) -> fn(f64) -> f64 {
        self.func.expect("Attempting to use an unresolved FuncExpr")
    }

    pub fn interval_func(&self) -> fn(&Interval) -> Interval {
        self.interval_func
            .expect("Attempting to use an unresolved FuncExpr")
    }

    pub fn set_func(&mut self, func: fn(f64) -> f64, interval_func: fn(&Interval) -> Interval) {
        self.func = Some(func);
        self.interval_func = Some(interval_func);
    }
}
