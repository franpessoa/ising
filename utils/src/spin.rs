use std::ops::{Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpinState {
    P,
    N,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spin {
    s: SpinState,
}

impl Spin {
    pub fn new(pos: bool) -> Self {
        if pos {
            return Self { s: SpinState::P };
        }

        Self { s: SpinState::N }
    }
}

impl Mul<f64> for Spin {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        let p = match self.s {
            SpinState::P => 1.0,
            SpinState::N => -1.0,
        };

        p * rhs
    }
}

impl Mul<Spin> for Spin {
    type Output = f64;

    fn mul(self, rhs: Spin) -> Self::Output {
        if self.s == rhs.s {
            return 1.0;
        }

        -1.0
    }
}

impl Neg for Spin {
    type Output = Spin;

    fn neg(self) -> Self {
        Self {
            s: match self.s {
                SpinState::P => SpinState::N,
                SpinState::N => SpinState::P,
            },
        }
    }
}

impl From<Spin> for f64 {
    fn from(value: Spin) -> Self {
        if value.s == SpinState::P {
            return 1.0;
        };
        -1.0
    }
}
