//! Structures and functions relating to the chess board

mod file;
mod rank;
mod square;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::{Color, NUM_COLORS, NUM_PIECES};

pub use self::file::File;
pub use self::rank::Rank;
pub use self::square::Square;

/// A integrer representation of a chess board.
///
/// Bitboards are an array of boolean flags stored as an integer. The
/// presence of a flag indicates the presence of a piece. Read more about
/// bitboards [here](https://www.chessprogramming.org/Bitboards).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BitBoard(u64);

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ self.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 ^ rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

pub struct Board {
    pieces_positions: [BitBoard; NUM_PIECES],
    color_positions: [BitBoard; NUM_COLORS],
    color_to_move: Color,
}
