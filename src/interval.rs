use std::{f64, ops};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    lower: f64,
    upper: f64,
    continuous: bool,
}

impl Interval {
    pub const EMPTY: Self = Self {
        lower: f64::NAN,
        upper: f64::NAN,
        continuous: false,
    };

    pub fn new(lower: f64, upper: f64) -> Self {
        Self::new_impl(lower, upper, true)
    }

    fn new_impl(lower: f64, upper: f64, continuous: bool) -> Self {
        assert!(lower <= upper);
        assert!(lower != f64::INFINITY && upper != f64::NEG_INFINITY);
        assert!(!lower.is_nan() && !upper.is_nan());

        Self {
            lower,
            upper,
            continuous,
        }
    }

    pub fn empty(&self) -> bool {
        self.lower.is_nan()
    }

    pub fn is_finite(&self) -> bool {
        !self.lower.is_infinite() && !self.upper.is_infinite()
    }

    pub fn continuous(&self) -> bool {
        self.continuous
    }

    pub fn lower(&self) -> f64 {
        assert!(!self.empty());
        self.lower
    }

    pub fn upper(&self) -> f64 {
        assert!(!self.empty());
        self.upper
    }

    pub fn sin(&self) -> Self {
        if self.empty() {
            return Self::EMPTY;
        }

        if !self.is_finite() {
            return Self::new_impl(-1.0, 1.0, self.continuous);
        }

        let mut ta = self.lower() / (2.0 * f64::consts::PI);
        let mut tb = self.upper() / (2.0 * f64::consts::PI);

        tb -= ta.floor();
        ta -= ta.floor();

        let max_one = ta <= 0.25 && tb >= 0.25 || tb >= 1.25;
        let min_neg_one = ta <= 0.75 && tb >= 0.75 || tb >= 1.75;

        if max_one && min_neg_one {
            Self::new_impl(-1.0, 1.0, self.continuous)
        } else if max_one {
            let a = self.lower().sin();
            let b = self.upper().sin();
            Self::new_impl(a.min(b), 1.0, self.continuous)
        } else if min_neg_one {
            let a = self.lower().sin();
            let b = self.upper().sin();
            Self::new_impl(-1.0, a.max(b), self.continuous)
        } else {
            let a = self.lower().sin();
            let b = self.upper().sin();
            Self::new_impl(a.min(b), a.max(b), self.continuous)
        }
    }

    pub fn cos(&self) -> Self {
        if self.empty() {
            return Self::EMPTY;
        }

        if !self.is_finite() {
            return Self::new_impl(-1.0, 1.0, self.continuous);
        }

        let mut ta = self.lower() / (2.0 * f64::consts::PI);
        let mut tb = self.upper() / (2.0 * f64::consts::PI);

        tb -= ta.floor();
        ta -= ta.floor();

        // technically ta == 0.0 is also a condition but it's basically impossible
        let max_one = tb >= 1.00;
        let min_neg_one = ta <= 0.5 && tb >= 0.5 || tb >= 1.5;

        if max_one && min_neg_one {
            Self::new_impl(-1.0, 1.0, self.continuous)
        } else if max_one {
            let a = self.lower().cos();
            let b = self.upper().cos();
            Self::new_impl(a.min(b), 1.0, self.continuous)
        } else if min_neg_one {
            let a = self.lower().cos();
            let b = self.upper().cos();
            Self::new_impl(-1.0, a.max(b), self.continuous)
        } else {
            let a = self.lower().cos();
            let b = self.upper().cos();
            Self::new_impl(a.min(b), a.max(b), self.continuous)
        }
    }

    pub fn tan(&self) -> Self {
        if self.empty() {
            return Self::EMPTY;
        }

        self.sin() / self.cos()
    }

    pub fn ln(&self) -> Self {
        if self.empty() || self.upper() <= 0.0 {
            return Self::EMPTY;
        }

        let min = if self.lower() <= 0.0 {
            f64::NEG_INFINITY
        } else {
            self.lower().ln()
        };

        Self::new_impl(
            min,
            self.upper().ln(),
            self.lower() > 0.0 && self.continuous,
        )
    }

    // pow is only defined on ((0, inf] x [-inf, inf]) U ([0, inf] x [0, inf])
    // equivalently pow is only defined if the base is positive or the base is nonnegative and the exponent is nonnegative
    pub fn pow(&self, rhs: &Self) -> Self {
        if self.empty() || rhs.empty() || self.upper() < 0.0 {
            return Self::EMPTY;
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        if self.upper() > 0.0 {
            if rhs.upper() > 0.0 {
                // pos^pos
                let a = self.upper().powf(rhs.upper());
                let b = self.upper().powf(rhs.lower().max(0.0));
                let c = self.lower().max(0.0).powf(rhs.upper());
                let d = self.lower().max(0.0).powf(rhs.lower().max(0.0));
                min = min.min(a).min(b).min(c).min(d);
                max = max.max(a).max(b).max(c).max(d);
            }

            if rhs.lower() <= 0.0 && rhs.upper() >= 0.0 {
                // pos^0
                min = min.min(1.0);
                max = max.max(1.0);
            }

            if rhs.lower() < 0.0 {
                // pos^neg
                let a = self.upper().powf(rhs.upper().min(0.0));
                let b = self.upper().powf(rhs.lower());
                let (c, d) = if self.lower() <= 0.0 {
                    (f64::INFINITY, f64::INFINITY)
                } else {
                    let c = self.lower().powf(rhs.upper().min(0.0));
                    let d = self.lower().powf(rhs.lower());
                    (c, d)
                };
                min = min.min(a).min(b).min(c).min(d);
                max = max.max(a).max(b).max(c).max(d);
            }
        }

        if self.lower() <= 0.0 && self.upper() >= 0.0 {
            if rhs.lower() <= 0.0 && rhs.upper() >= 0.0 {
                // 0^0 = 1
                min = min.min(1.0);
                max = max.max(1.0);
            }

            if rhs.upper() > 0.0 {
                // 0^pos = 0
                min = min.min(0.0);
                max = max.max(0.0);
            }
        }

        if min == f64::INFINITY || max == f64::NEG_INFINITY {
            Self::EMPTY
        } else {
            Interval::new_impl(
                min,
                max,
                self.continuous && rhs.continuous && (self.lower() > 0.0 || rhs.lower() > 0.0),
            )
        }
    }

    /*fn powf(&self) -> Self {
        // ?????????
    }*/
}

impl ops::Add<Interval> for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.empty() || rhs.empty() {
            return Self::EMPTY;
        }

        Self::new_impl(
            self.lower() + rhs.lower(),
            self.upper() + rhs.upper(),
            self.continuous && rhs.continuous,
        )
    }
}

impl ops::Sub<Interval> for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.empty() || rhs.empty() {
            return Self::EMPTY;
        }

        Self::new_impl(
            self.lower() - rhs.upper(),
            self.upper() - rhs.lower(),
            self.continuous && rhs.continuous,
        )
    }
}

fn interval_mul(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 { 0.0 } else { a * b }
}

impl ops::Mul<Interval> for Interval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.empty() || rhs.empty() {
            return Self::EMPTY;
        }

        let a = interval_mul(self.lower(), rhs.lower());
        let b = interval_mul(self.lower(), rhs.upper());
        let c = interval_mul(self.upper(), rhs.lower());
        let d = interval_mul(self.upper(), rhs.upper());
        Self::new_impl(
            a.min(b).min(c).min(d),
            a.max(b).max(c).max(d),
            self.continuous && rhs.continuous,
        )
    }
}

impl ops::Div<Interval> for Interval {
    type Output = Self;

    fn div(self, rhs: Interval) -> Self::Output {
        if self.empty() || rhs.empty() {
            return Self::EMPTY;
        }

        if rhs.lower() == 0.0 && rhs.upper() == 0.0 {
            return Self::EMPTY;
        }

        if self.lower() == 0.0 && self.upper() == 0.0 {
            return Self::new_impl(
                0.0,
                0.0,
                (rhs.lower() > 0.0 || rhs.upper() < 0.0) && self.continuous && rhs.continuous,
            );
        }

        if rhs.lower() < 0.0 && rhs.upper() > 0.0 {
            Self::new_impl(f64::NEG_INFINITY, f64::INFINITY, false)
        } else if rhs.lower() >= 0.0 {
            if rhs.lower() == 0.0 {
                let a = interval_mul(self.lower(), f64::INFINITY);
                let b = interval_mul(self.upper(), f64::INFINITY);
                let c = self.lower() / rhs.upper();
                let d = self.upper() / rhs.upper();
                Self::new_impl(a.min(b).min(c).min(d), a.max(b).max(c).max(d), false)
            } else {
                let a = self.lower() / rhs.lower();
                let b = self.lower() / rhs.upper();
                let c = self.upper() / rhs.lower();
                let d = self.upper() / rhs.upper();
                Self::new_impl(
                    a.min(b).min(c).min(d),
                    a.max(b).max(c).max(d),
                    self.continuous && rhs.continuous,
                )
            }
        } else {
            assert!(rhs.upper() <= 0.0);
            if rhs.upper() == 0.0 {
                let a = interval_mul(self.lower(), f64::NEG_INFINITY);
                let b = interval_mul(self.upper(), f64::NEG_INFINITY);
                let c = self.lower() / rhs.lower();
                let d = self.upper() / rhs.lower();
                Self::new_impl(a.min(b).min(c).min(d), a.max(b).max(c).max(d), false)
            } else {
                let a = self.lower() / rhs.lower();
                let b = self.lower() / rhs.upper();
                let c = self.upper() / rhs.lower();
                let d = self.upper() / rhs.upper();
                Self::new_impl(
                    a.min(b).min(c).min(d),
                    a.max(b).max(c).max(d),
                    self.continuous && rhs.continuous,
                )
            }
        }
    }
}

impl ops::Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new_impl(-self.upper(), -self.lower(), self.continuous)
    }
}
