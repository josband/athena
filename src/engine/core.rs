use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const PAWN_VALUE: Evaluation = Evaluation(100);
pub const KNIGHT_VALUE: Evaluation = Evaluation(300);
pub const BISHOP_VALUE: Evaluation = Evaluation(300);
pub const ROOK_VALUE: Evaluation = Evaluation(500);
pub const QUEEN_VALUE: Evaluation = Evaluation(1000);
pub const KING_VALUE: Evaluation = Evaluation(10000);

/// Evaluation of a chess position.
///
/// The sign of the value can be with respect to the side to move
/// in a position, or it can be based on a fixed size where white
/// is positive and black is negative. '1' represents 1/100th of
/// a pawn (called a centipawn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Evaluation(i32);

impl Evaluation {
    pub const MAX: Evaluation = Evaluation(i32::MAX);
    pub const MIN: Evaluation = Evaluation(i32::MIN);
    pub const EQUAL: Evaluation = Evaluation(0);

    pub fn new(val: i32) -> Self {
        Self(val)
    }
}

impl Neg for Evaluation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Evaluation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Evaluation {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Evaluation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Evaluation {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Evaluation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Evaluation {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Mul<i32> for Evaluation {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<i32> for Evaluation {
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs
    }
}

impl Div for Evaluation {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Evaluation {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Div<i32> for Evaluation {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<i32> for Evaluation {
    fn div_assign(&mut self, rhs: i32) {
        self.0 /= rhs
    }
}
