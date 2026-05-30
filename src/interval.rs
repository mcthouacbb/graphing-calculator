use std::{f64, ops};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
    continuous: bool,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f64::NAN,
        max: f64::NAN,
        continuous: false,
    };

    fn new(min: f64, max: f64, continuous: bool) -> Self {
        assert!(min <= max);
        assert!(min != f64::INFINITY && max != f64::NEG_INFINITY);
        assert!(!min.is_nan() && !max.is_nan());

        Self {
            min,
            max,
            continuous,
        }
    }

    pub fn empty(&self) -> bool {
        self.min.is_nan()
    }

    pub fn is_finite(&self) -> bool {
        !self.min.is_infinite() && !self.max.is_infinite()
    }

    pub fn continuous(&self) -> bool {
        self.continuous
    }

    pub fn min(&self) -> f64 {
        assert!(!self.empty());
        self.min
    }

    pub fn max(&self) -> f64 {
        assert!(!self.empty());
        self.max
    }

    pub fn sin(&self) -> Self {
        if self.empty() {
            return Self::EMPTY;
        }

        if !self.is_finite() {
            return Self::new(-1.0, 1.0, self.continuous);
        }

        let mut ta = self.min() / (2.0 * f64::consts::PI);
        let mut tb = self.max() / (2.0 * f64::consts::PI);

        tb -= ta.floor();
        ta -= ta.floor();

        let max_one = ta <= 0.25 && tb >= 0.25 || tb >= 1.25;
        let min_neg_one = ta <= 0.75 && tb >= 0.75 || tb >= 1.75;

        if max_one && min_neg_one {
            Self::new(-1.0, 1.0, self.continuous)
        } else if max_one {
            let a = self.min().sin();
            let b = self.max().sin();
            Self::new(a.min(b), 1.0, self.continuous)
        } else if min_neg_one {
            let a = self.min().sin();
            let b = self.max().sin();
            Self::new(-1.0, a.max(b), self.continuous)
        } else {
            let a = self.min().sin();
            let b = self.max().sin();
            Self::new(a.min(b), a.max(b), self.continuous)
        }
    }

    pub fn cos(&self) -> Self {
        if self.empty() {
            return Self::EMPTY;
        }

        if !self.is_finite() {
            return Self::new(-1.0, 1.0, self.continuous);
        }

        let mut ta = self.min() / (2.0 * f64::consts::PI);
        let mut tb = self.max() / (2.0 * f64::consts::PI);

        tb -= ta.floor();
        ta -= ta.floor();

        // technically ta == 0.0 is also a condition but it's basically impossible
        let max_one = tb >= 1.00;
        let min_neg_one = ta <= 0.5 && tb >= 0.5 || tb >= 1.5;

        if max_one && min_neg_one {
            Self::new(-1.0, 1.0, self.continuous)
        } else if max_one {
            let a = self.min().cos();
            let b = self.max().cos();
            Self::new(a.min(b), 1.0, self.continuous)
        } else if min_neg_one {
            let a = self.min().cos();
            let b = self.max().cos();
            Self::new(-1.0, a.max(b), self.continuous)
        } else {
            let a = self.min().cos();
            let b = self.max().cos();
            Self::new(a.min(b), a.max(b), self.continuous)
        }
    }

    pub fn tan(&self) -> Self {
        if self.empty() {
            return Self::EMPTY;
        }

        self.sin() / self.cos()
    }

    pub fn ln(&self) -> Self {
        if self.empty() || self.max() <= 0.0 {
            return Self::EMPTY;
        }

        let min = if self.min() <= 0.0 {
            f64::NEG_INFINITY
        } else {
            self.min().ln()
        };

        Self::new(min, self.max().ln(), self.min() > 0.0 && self.continuous)
    }

    // pow is only defined on ((0, inf] x [-inf, inf]) U ([0, inf] x [0, inf])
    // equivalently pow is only defined if the base is positive or the base is nonnegative and the exponent is nonnegative
    fn pow(&self, rhs: &Self) -> Self {
        if self.empty() || rhs.empty() || self.max < 0.0 {
            return Self::EMPTY;
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        if self.max() > 0.0 {
            if rhs.max() > 0.0 {
                // pos^pos
                let a = self.max().powf(rhs.max());
                let b = self.max().powf(rhs.min().max(0.0));
                let c = self.min().max(0.0).powf(rhs.max());
                let d = self.min().max(0.0).powf(rhs.min().max(0.0));
                min = min.min(a).min(b).min(c).min(d);
                max = max.max(a).max(b).max(c).max(d);
            }

            if rhs.min() <= 0.0 && rhs.max() >= 0.0 {
                // pos^0
                min = min.min(1.0);
                max = max.max(1.0);
            }

            if rhs.min() < 0.0 {
                // pos^neg
                let a = self.max().powf(rhs.max().min(0.0));
                let b = self.max().powf(rhs.min());
                let (c, d) = if self.min() <= 0.0 {
                    (f64::INFINITY, f64::INFINITY)
                } else {
                    let c = self.min().powf(rhs.max().min(0.0));
                    let d = self.min().powf(rhs.min());
                    (c, d)
                };
                min = min.min(a).min(b).min(c).min(d);
                max = max.max(a).max(b).max(c).max(d);
            }
        }

        if self.min <= 0.0 && self.max >= 0.0 {
            if rhs.min <= 0.0 && rhs.max >= 0.0 {
                // 0^0 = 1
                min = min.min(1.0);
                max = max.max(1.0);
            }

            if rhs.max > 0.0 {
                // 0^pos = 0
                min = min.min(0.0);
                max = max.max(0.0);
            }
        }

        if min == f64::INFINITY || max == f64::NEG_INFINITY {
            Self::EMPTY
        } else {
            Interval::new(
                min,
                max,
                self.continuous && rhs.continuous && (self.min() > 0.0 || rhs.min() > 0.0),
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

        Self::new(
            self.min() + rhs.min(),
            self.max() + rhs.max(),
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

        Self::new(
            self.min() - rhs.max(),
            self.max() - rhs.min(),
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

        let a = interval_mul(self.min(), rhs.min());
        let b = interval_mul(self.min(), rhs.max());
        let c = interval_mul(self.max(), rhs.min());
        let d = interval_mul(self.max(), rhs.max());
        Self::new(
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

        if rhs.min() == 0.0 && rhs.max() == 0.0 {
            return Self::EMPTY;
        }

        if self.min() == 0.0 && self.max() == 0.0 {
            return Self::new(
                0.0,
                0.0,
                (rhs.min() > 0.0 || rhs.max() < 0.0) && self.continuous && rhs.continuous,
            );
        }

        if rhs.min() < 0.0 && rhs.max() > 0.0 {
            Self::new(f64::NEG_INFINITY, f64::INFINITY, false)
        } else if rhs.min() >= 0.0 {
            if rhs.min() == 0.0 {
                let a = interval_mul(self.min(), f64::INFINITY);
                let b = interval_mul(self.max(), f64::INFINITY);
                let c = self.min() / rhs.max();
                let d = self.max() / rhs.max();
                Self::new(a.min(b).min(c).min(d), a.max(b).max(c).max(d), false)
            } else {
                let a = self.min() / rhs.min();
                let b = self.min() / rhs.max();
                let c = self.max() / rhs.min();
                let d = self.max() / rhs.max();
                Self::new(
                    a.min(b).min(c).min(d),
                    a.max(b).max(c).max(d),
                    self.continuous && rhs.continuous,
                )
            }
        } else {
            assert!(rhs.max() <= 0.0);
            if rhs.max() == 0.0 {
                let a = interval_mul(self.min(), f64::NEG_INFINITY);
                let b = interval_mul(self.max(), f64::NEG_INFINITY);
                let c = self.min() / rhs.min();
                let d = self.max() / rhs.min();
                Self::new(a.min(b).min(c).min(d), a.max(b).max(c).max(d), false)
            } else {
                let a = self.min() / rhs.min();
                let b = self.min() / rhs.max();
                let c = self.max() / rhs.min();
                let d = self.max() / rhs.max();
                Self::new(
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
        Self::new(-self.max(), -self.min(), self.continuous)
    }
}
